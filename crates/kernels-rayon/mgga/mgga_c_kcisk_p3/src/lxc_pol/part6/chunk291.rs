//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 291/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk291(t1249: f64, t397: f64, t539: f64, t535: f64, t533: f64, t1308: f64, sigma0: f64) -> (f64, f64, f64) {
    let t1576 = t397 * t1249 * t539;
    let t1578 = 0.89953943580886586067e-2_f64 * t535 * t1576;
    let t1579 = t533 * sigma0;
    let t1580 = t1579 * t1308;
    (t1576, t1578, t1580)
}
