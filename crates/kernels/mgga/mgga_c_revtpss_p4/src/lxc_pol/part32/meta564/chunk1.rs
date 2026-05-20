//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1887/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1887<F: Float>(t2689: F, t27239: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t1955: F, t27198: F, t2769: F) -> (F, F, F, F, F) {
    let t99091 = t2689 * t27239;
    let t99099 = t25277 * t4458;
    let t99102 = t7021 * t14685 * t14756;
    let t99113 = t93015 * t14760;
    let t99191 = t1955 * t27198 * t2769;
    (t99091, t99099, t99102, t99113, t99191)
}
