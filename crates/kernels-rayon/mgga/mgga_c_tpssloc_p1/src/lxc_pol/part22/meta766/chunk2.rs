//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2590/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590(t19026: f64, t4997: f64, t18975: f64, t5005: f64, t11719: f64, t22307: f64, t248: f64, t3570: f64, t11668: f64, t1213: f64, t1214: f64, t1737: f64, t19002: f64, t3577: f64, t4724: f64, t475: f64, t52879: f64, t6219: f64, t65479: f64, t65482: f64, t65485: f64, t65506: f64, t65957: f64, t72181: f64, t72183: f64, t72217: f64) -> f64 {
    let t72223 = t19026 * t4997;
    let t72225 = t5005 * t18975;
    let t72229 = t11719 * t248 * t3570 * t22307;
    let t72233 = t72181 / 1536.0_f64 - t72183 / 2304.0_f64 + t65957 * t1737 / 1024.0_f64 - t65479 / 1152.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t6219 * t4724 + t65482 / 1152.0_f64 - t65485 / 576.0_f64 - t65506 / 576.0_f64 + t1213 * t248 * t1214 * t72217 * t475 / 3072.0_f64 + 19.0_f64 / 864.0_f64 * t72223 + 5.0_f64 / 6912.0_f64 * t72225 + t72229 / 768.0_f64 - t52879 * t19002 / 384.0_f64;
    t72233
}
