//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 785/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk785(t8796: f64, t8802: f64, t8805: f64, t8809: f64, t8813: f64, t8815: f64, t8818: f64, t2019: f64, t2323: f64, t7926: f64, t7487: f64, t8346: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38300 = 0.39914139006212695214e-1_f64 * t8796;
    let t38304 = 0.79828278012425390428e-1_f64 * t8802;
    let t38305 = 0.79828278012425390428e-1_f64 * t8805;
    let t38306 = 0.10215503974391481455e-3_f64 * t8809;
    let t38307 = 0.25538759935978703638e-4_f64 * t8813;
    let t38308 = 0.25538759935978703638e-4_f64 * t8815;
    let t38310 = 0.4726e1_f64 * t8818;
    let t38312 = t2019 * t7926 * t2323;
    let t38314 = t7487 * t8346;
    (t38300, t38304, t38305, t38306, t38307, t38308, t38310, t38312, t38314)
}
