//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3013/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013(t1058: f64, t1060: f64, t11051: f64, t14526: f64, t14600: f64, t14618: f64, t1615: f64, t18086: f64, t18099: f64, t18155: f64, t23508: f64, t3040: f64, t3180: f64, t3197: f64, t3200: f64, t3201: f64, t360: f64, t43503: f64, t43515: f64, t43516: f64, t43576: f64, t43577: f64, t4594: f64, t4649: f64, t4674: f64, t4684: f64, t4685: f64, t50465: f64, t50509: f64, t50516: f64, t50592: f64, t5928: f64, t5937: f64, t62925: f64) -> f64 {
    let t63058 = 24.0_f64 * t43576 * t5928 * t43577 * t3040 + t18086 * t3197 + t11051 * t5937 - 24.0_f64 * t50516 * t50509 * t4594 * t4649 + 14.0_f64 * t43515 * t5928 * t43516 * t3040 + 2.0_f64 * t3180 * t18155 - 2.0_f64 * t3200 * t18099 * t4684 + 8.0_f64 * t14618 * t14600 + 2.0_f64 * t1058 * t14526 * t1615 * t1060 - t43503 * t5928 * t23508 * t3040 * t360 - 4.0_f64 * t50592 * t4685 - t3200 * t62925 * t3201 + 8.0_f64 * t50465 * t4674;
    t63058
}
