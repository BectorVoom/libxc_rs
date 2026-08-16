//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 872/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk872(t14225: f64, t7248: f64, t8421: f64, t8426: f64, t9188: f64, t3352: f64, t8431: f64, t3157: f64, t33235: f64, t15310: f64, t52781: f64, t10570: f64, t1652: f64, t262: f64, t3068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75629 = t14225 * t7248 * t8421;
    let t75632 = t14225 * t9188 * t8426;
    let t75635 = t14225 * t3352 * t8431;
    let t75638 = t33235 * t3157;
    let t75640 = t52781 * t15310;
    let t75644 = t10570 * t3068 * t262 * t1652;
    (t75629, t75632, t75635, t75638, t75640, t75644)
}
