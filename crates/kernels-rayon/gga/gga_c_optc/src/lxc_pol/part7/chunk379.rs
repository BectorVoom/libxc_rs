//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 379/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk379(t1810: f64, t1828: f64, t1772: f64, t31: f64, t4: f64, t508: f64, t514: f64, t209: f64, t535: f64, t580: f64, t579: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1829 = t1810 * t1828;
    let t1834 = 0.14764770444444444444e-2_f64 * t4 * t1772 * t31;
    let t1835 = t508 * t514;
    let t1838 = 0.35616666666666666667e-1_f64 * t209 * t1835 * t535;
    let t1842 = t508 * t580;
    let t1846 = t579 * t80;
    (t1829, t1834, t1835, t1838, t1842, t1846)
}
