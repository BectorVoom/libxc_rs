//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2592/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2592(t11712: f64, t11880: f64, t491: f64, t1734: f64, t6739: f64, t3609: f64, t52434: f64, t3507: f64, t5052: f64, t1215: f64, t2250: f64, t475: f64) -> (f64, f64, f64, f64, f64) {
    let t52479 = t11712 * t11880 * t491;
    let t52480 = t1734 * t6739;
    let t52485 = t52434 * t3609;
    let t52500 = t5052 * t3507;
    let t52531 = t2250 * t1215;
    let t52532 = t52531 * t475;
    (t52479, t52480, t52485, t52500, t52532)
}
