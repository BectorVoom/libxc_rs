//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1431/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1431(t122423: f64, t122438: f64, t1799: f64, t2085: f64, t1307: f64, t26331: f64, t26446: f64, t1992: f64, t550: f64, t6976: f64, t93501: f64, t22704: f64, t22705: f64, t33280: f64) -> (f64, f64, f64, f64, f64) {
    let t122439 = t122423 + t122438;
    let t122448 = t2085 * t1799;
    let t122451 = t26331 * t26446 * t122448 * t1307;
    let t122457 = t1992 * t6976 * t93501 * t550;
    let t122460 = t22704 * t22705 * t33280;
    (t122439, t122448, t122451, t122457, t122460)
}
