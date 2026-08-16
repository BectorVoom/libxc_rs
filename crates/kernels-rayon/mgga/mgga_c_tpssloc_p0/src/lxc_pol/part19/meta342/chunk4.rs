//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1223/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1223(t116: f64, t786: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t13005: f64, t213: f64, t221: f64, t2379: f64, t2553: f64, t41187: f64, t41190: f64, t41192: f64, t41194: f64, t41197: f64, t41200: f64, t41203: f64, t41205: f64, t41209: f64, t41212: f64, t4127: f64, t9516: f64) -> f64 {
    let t41214 = t9534 * t786 * t116;
    let t41217 = t41214 * t133 * t6600 * t776;
    let t41229 = -0.77777777777777777775e-1_f64 * t41187 + 0.13148148148148148148e0_f64 * t41190 - 0.31666666666666666666e-1_f64 * t41192 + 0.23333333333333333332e0_f64 * t41194 + 0.94999999999999999997e-1_f64 * t41197 - t41200 - 0.29999999999999999998e-1_f64 * t41203 - 0.13999999999999999999e0_f64 * t41205 + t41209 + t41212 + 0.11111111111111111111e-2_f64 * t41217 + 0.19999999999999999999e-1_f64 * t4127 * t221 * t213 * t9516 * t776 - 0.11999999999999999999e0_f64 * t13005 * t221 * t213 * t2379 * t2553;
    t41229
}
