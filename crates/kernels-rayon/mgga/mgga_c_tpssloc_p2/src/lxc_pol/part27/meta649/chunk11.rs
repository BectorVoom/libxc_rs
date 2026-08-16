//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2261/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261(t23858: f64, t7685: f64, t22607: f64, t7688: f64, t1390: f64, t16018: f64, t1983: f64, t6878: f64, t22574: f64, t56194: f64, t8643: f64, t113: f64, t1393: f64, t1459: f64, t26138: f64, t4072: f64, t5107: f64, t6515: f64, t652: f64, t6862: f64, t83935: f64, t86673: f64, t86676: f64, t86679: f64, t86682: f64, t86684: f64, t86688: f64, t86693: f64, t86698: f64, t86700: f64, t86702: f64, t89836: f64, t90016: f64) -> f64 {
    let t90020 = 2.0_f64 * t7685 * t23858;
    let t90022 = 3.0_f64 * t22607 * t7688;
    let t90023 = t1390 * t16018;
    let t90026 = 3.0_f64 * t1983 * t6878 * t90023;
    let t90029 = 6.0_f64 * t22574 * t8643 * t56194;
    let t90030 = 2.0_f64 * t26138 * t1393 + t86673 + t86676 + t86679 - 2.0_f64 * t6515 * t5107 + t86682 - t86684 - t86688 - 2.0_f64 * t83935 * t1459 + t86693 - 4.0_f64 * t652 * t6862 * t4072 - t86698 - t86700 - t86702 - t113 * (t89836 + t90016) + t90020 + t90022 + t90026 - t90029;
    t90030
}
