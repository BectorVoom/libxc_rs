//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1840/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1840<F: Float>(t25410: F, t93320: F, t7063: F, t860: F, t25374: F, t11007: F, t1955: F, t7056: F, t93189: F, t93169: F, t1113: F, t2411: F) -> (F, F, F, F, F, F, F, F) {
    let t93321 = t93320 * t25410;
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93349 = t1955 * t7056 * t11007;
    let t93364 = t93320 * t25374;
    let t93371 = t93189 * t25410;
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t94245 = t2411 * t1113;
    (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245)
}
