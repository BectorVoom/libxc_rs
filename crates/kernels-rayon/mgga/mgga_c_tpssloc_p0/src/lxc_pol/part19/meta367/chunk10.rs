//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1354/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1354(t1036: f64, t10361: f64, t1031: f64, t10360: f64, t10403: f64, t1041: f64, t10413: f64, t10419: f64, t1044: f64, t10937: f64, t10970: f64, t248: f64, t2780: f64, t3041: f64, t3071: f64, t3077: f64, t3088: f64, t3132: f64, t378: f64, t41640: f64, t41688: f64, t43143: f64, t43155: f64, t43157: f64, t43161: f64, t43167: f64) -> f64 {
    let t43176 = t10361 * t1036;
    let t43181 = -t43143 / 54.0_f64 + t10937 * t10419 / 36.0_f64 + t10403 * t3071 * t3132 * t2780 / 384.0_f64 - t10413 * t3071 * t3041 * t2780 / 768.0_f64 - 11.0_f64 / 81.0_f64 * t43155 - 10.0_f64 / 243.0_f64 * t43157 - t43161 / 2304.0_f64 - t1041 * t248 * t1044 * t41640 / 768.0_f64 + t43167 / 192.0_f64 - 5.0_f64 / 432.0_f64 * t1041 * t248 * t10970 * t41688 - t10360 * t1031 * t378 / 144.0_f64 + t43176 / 1152.0_f64 + 19.0_f64 / 288.0_f64 * t3077 * t3088 * t378;
    t43181
}
