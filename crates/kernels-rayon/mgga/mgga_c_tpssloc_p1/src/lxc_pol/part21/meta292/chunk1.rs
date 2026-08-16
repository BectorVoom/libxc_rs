//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1607/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1607(t2843: f64, t290: f64, t2924: f64, t2932: f64, t2860: f64, t919: f64, t2904: f64, t938: f64, t10629: f64, t315: f64) -> (f64, f64, f64, f64, f64) {
    let t10704 = 1.0_f64 / t2843 / t290;
    let t10723 = t2924 * t2932;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    (t10704, t10723, t10740, t10747, t10756)
}
