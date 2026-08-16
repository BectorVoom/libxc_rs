//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 751/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk751(t23110: f64, t232: f64, t236: f64, t828: f64, t23109: f64, t1898: f64, t2613: f64, t249: f64, t6609: f64, t838: f64, t6589: f64, t6597: f64) -> (f64, f64, f64, f64, f64) {
    let t23113 = t23110 * t236 * t828 * t232;
    let t23114 = t23109 * t23113;
    let t23116 = t2613 * t1898;
    let t23117 = t23116 * t249;
    let t23119 = t6609 * t838;
    let t23120 = 7.0_f64 / 1152.0_f64 * t23119;
    let t23121 = t6597 * t6589;
    (t23114, t23117, t23119, t23120, t23121)
}
