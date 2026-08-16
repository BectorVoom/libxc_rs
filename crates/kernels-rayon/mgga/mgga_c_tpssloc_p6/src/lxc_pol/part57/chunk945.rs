//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 945/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk945(t1799: f64, t2085: f64, t22704: f64, t22705: f64, t33280: f64, t33281: f64, t6914: f64, t1338: f64, t33266: f64, t33285: f64, t6883: f64, t33284: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122448 = t2085 * t1799;
    let t122460 = t22704 * t22705 * t33280;
    let t122462 = t6914 * t33281;
    let t122475 = t1338 * t33266;
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    (t122448, t122460, t122462, t122475, t122503, t122507)
}
