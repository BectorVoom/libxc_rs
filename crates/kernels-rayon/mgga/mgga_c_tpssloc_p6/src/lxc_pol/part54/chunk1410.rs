//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1410/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1410(t113: f64, t121958: f64, t122082: f64, t33363: f64, t6880: f64, t2018: f64, t26161: f64, t26558: f64, t5356: f64, t33273: f64, t81159: f64, t115545: f64, t22633: f64, t26215: f64) -> (f64, f64, f64, f64, f64) {
    let t122084 = t113 * (t121958 + t122082);
    let t122088 = 3.0_f64 * t33363 * t6880;
    let t122094 = 2.0_f64 * t26161 * t26558 * t2018 * t5356;
    let t122102 = t81159 * t33273;
    let t122107 = t22633 * t115545 * t26215;
    (t122084, t122088, t122094, t122102, t122107)
}
