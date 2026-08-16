//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1355/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1355(t1509: f64, t23097: f64, t232: f64, t5544: f64, t815: f64, t1484: f64, t5612: f64, t2628: f64, t5585: f64, t20887: f64, t23146: f64, t5593: f64, t87199: f64) -> (f64, f64, f64, f64, f64) {
    let t105278 = t23097 * t815 * t5544 * t1509 * t232;
    let t105282 = t23097 * t815 * t5612 * t1484;
    let t105286 = t23097 * t2628 * t5585 * t1484;
    let t105288 = t23146 * t20887;
    let t105290 = t87199 * t5593;
    (t105278, t105282, t105286, t105288, t105290)
}
