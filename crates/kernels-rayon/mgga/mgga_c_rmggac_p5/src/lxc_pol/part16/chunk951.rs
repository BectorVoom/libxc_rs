//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 951/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk951(t36457: f64, t9835: f64, t1469: f64, t3369: f64, t39851: f64, t559: f64, t2412: f64, t8582: f64, t2191: f64, t9790: f64, t9938: f64, t10040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45832 = t36457 * t9835;
    let t45836 = t39851 * t3369 * t559 * t1469;
    let t45844 = t2412 * t8582;
    let t45846 = t2191 * t9790;
    let t45864 = t2191 * t9938;
    let t45866 = t2191 * t10040;
    (t45832, t45836, t45844, t45846, t45864, t45866)
}
