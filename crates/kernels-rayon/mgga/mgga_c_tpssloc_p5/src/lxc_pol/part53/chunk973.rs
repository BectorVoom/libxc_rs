//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 973/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk973(t120124: f64, t12725: f64, t8326: f64, t1385: f64, t1799: f64, t31169: f64, t5234: f64, t31172: f64, t114002: f64, t32721: f64, t16242: f64, t31170: f64, t5248: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120125 = 2.0_f64 * t120124;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0_f64 * t120130;
    let t120240 = t1799 * t1385;
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    let t120344 = t114002 * t32721;
    let t120348 = t31170 * t5248 * t16242 * t550;
    (t120125, t120131, t120240, t120342, t120344, t120348)
}
