//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 538/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk538(t5330: f64, t79: f64, t5283: f64, t718: f64, t41: f64, t719: f64, t6973: f64, t740: f64, t1871: f64, t2558: f64, t1646: f64, t725: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7311 = t79 * t5330;
    let t7315 = t5283 * t718;
    let t7316 = t41 * t719;
    let t7320 = t6973 * t740;
    let t7336 = t2558 * t1871;
    let t7337 = t7336 * sigma2;
    let t7349 = t725 * t1646;
    (t7311, t7315, t7316, t7320, t7336, t7337, t7349)
}
