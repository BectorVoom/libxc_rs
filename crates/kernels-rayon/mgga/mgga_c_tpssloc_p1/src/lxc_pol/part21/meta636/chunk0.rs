//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2421/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2421(t2681: f64, t9671: f64, t2628: f64, t2690: f64, t812: f64, t2635: f64, t2629: f64, t9612: f64, t2617: f64, t9666: f64, t2379: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41373 = t9671 * t2681;
    let t41385 = t812 * t2628 * t2690;
    let t41386 = t41385 * t2635;
    let t41410 = t9612 * t2629;
    let t41424 = t2617 * t9666;
    let t41448 = t2632 * t2379;
    (t41373, t41385, t41386, t41410, t41424, t41448)
}
