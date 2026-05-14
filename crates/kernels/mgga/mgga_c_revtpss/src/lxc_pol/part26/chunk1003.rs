//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1003/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1003<F: Float>(t25410: F, t93320: F, t7063: F, t860: F, t25374: F, t11007: F, t1955: F, t7056: F, t231: F, t2771: F, t836: F, t10867: F, t867: F, t93189: F, t93169: F, t25759: F, t51806: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93321 = t93320 * t25410;
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93349 = t1955 * t7056 * t11007;
    let t93351 = t2771 * t836 * t231;
    let t93355 = t867 * t10867;
    let t93364 = t93320 * t25374;
    let t93371 = t93189 * t25410;
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t94228 = t25759 * t51806;
    (t93321, t93342, t93349, t93351, t93355, t93364, t93371, t93374, t93377, t94228)
}
