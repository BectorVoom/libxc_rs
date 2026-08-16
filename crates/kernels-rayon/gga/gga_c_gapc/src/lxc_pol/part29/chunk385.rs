//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 385/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk385(t435: f64, t507: f64, t561: f64, t589: f64, t195: f64, t588: f64, t169: f64, t1036: f64, t1037: f64, t457: f64, t505: f64, t202: f64) -> (f64, f64, f64, f64, f64) {
    let t1823 = t435 * t507;
    let t1826 = t561 * t589;
    let t1829 = t588 * t195;
    let t1830 = t169 * t1829;
    let t1835 = t1036 * t1037 * t457 * t505;
    let t1838 = t202 * t202;
    (t1823, t1826, t1830, t1835, t1838)
}
