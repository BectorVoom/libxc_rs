//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2147/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147(t52367: f64, t3030: f64, t4940: f64, t3623: f64, t11712: f64, t11880: f64, t491: f64, t1734: f64, t6739: f64, t3609: f64, t3242: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52368 = 0.18518518518518518518e-3_f64 * t52367;
    let t52434 = t4940 * t3030;
    let t52435 = t52434 * t3623;
    let t52479 = t11712 * t11880 * t491;
    let t52480 = t1734 * t6739;
    let t52485 = t52434 * t3609;
    let t52548 = t475 * t3242;
    (t52368, t52434, t52435, t52479, t52480, t52485, t52548)
}
