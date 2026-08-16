//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 785/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk785(t36527: f64, t1347: f64, t2232: f64, t4793: f64, t703: f64, t36700: f64, t36752: f64, t36796: f64, t36801: f64, t36942: f64, t36983: f64, t37017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37976 = 0.2439011983326002265e-2_f64 * t36527;
    let t38029 = t1347 * t2232;
    let t38031 = t4793 * t703;
    let t38047 = 0.18292589874945016987e-2_f64 * t36700;
    let t38060 = 0.30487649791575028312e-3_f64 * t36752;
    let t38079 = 0.2439011983326002265e-2_f64 * t36796;
    let t38080 = 0.11709622077411463733e-2_f64 * t36801;
    let t38123 = 0.26021382394247697185e-3_f64 * t36942;
    let t38140 = 0.13911401682674235141e-1_f64 * t36983;
    let t38149 = 0.28691693261408173224e-3_f64 * t37017;
    (t37976, t38029, t38031, t38047, t38060, t38079, t38080, t38123, t38140, t38149)
}
