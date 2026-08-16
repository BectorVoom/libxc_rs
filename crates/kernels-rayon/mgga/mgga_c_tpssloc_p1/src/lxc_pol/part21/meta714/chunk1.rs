//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2553/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2553(t13969: f64, t14098: f64, t3039: f64, t10224: f64, t4343: f64, t973: f64, t3130: f64, t4595: f64, t49850: f64, t10402: f64, t14618: f64, t14608: f64) -> (f64, f64, f64, f64, f64) {
    let t49897 = t3039 * t13969 * t14098;
    let t49906 = t973 * t10224 * t4343;
    let t49922 = t3130 * t49850 * t4595;
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    (t49897, t49906, t49922, t49929, t49934)
}
