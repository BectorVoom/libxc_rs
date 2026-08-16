//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1323/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1323(t1616: f64, t2011: f64, t3873: f64, t10529: f64, t10544: f64, t10791: f64, t11046: f64, t11155: f64, t1125: f64, t12483: f64, t12570: f64, t15436: f64, t2464: f64, t31777: f64, t38082: f64, t38086: f64, t38088: f64, t38093: f64, t38503: f64, t3883: f64, t7056: f64) -> (f64, f64, f64) {
    let t38508 = 2.0_f64 * t1616 * t3873 * t2011;
    let t38514 = 4.0_f64 * t10529 * t10544;
    let t38515 = 8.0_f64 * t10791 * t11046 + 4.0_f64 * t11046 * t11155 - 2.0_f64 * t1125 * t31777 + 4.0_f64 * t12483 * t7056 - 2.0_f64 * t12570 * t2464 + 2.0_f64 * t15436 * t3883 + t38082 + t38086 + t38088 - t38093 + t38503 - t38508 - t38514;
    (t38508, t38514, t38515)
}
