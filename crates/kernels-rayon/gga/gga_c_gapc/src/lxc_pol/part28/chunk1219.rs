//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1219/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1219(t1036: f64, t11311: f64, t13738: f64, t5856: f64, t11496: f64, t185: f64, t9386: f64, t11435: f64, t129: f64, t21778: f64, t11434: f64, t26331: f64, t5544: f64) -> (f64, f64, f64, f64) {
    let t34394 = t5856 * t11311 * t1036 * t13738;
    let t34397 = t185 * t9386 * t11496;
    let t34400 = t21778 * t129 * t11435;
    let t34403 = t11434 * t26331 * t5544;
    (t34394, t34397, t34400, t34403)
}
