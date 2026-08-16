//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 709/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk709(t2764: f64, t919: f64, t923: f64, t307: f64, t922: f64) -> (f64, f64, f64, f64) {
    let t2848 = 0.22831111111111111111e-1_f64 * t2764;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    let t2860 = 1.0_f64 / t2859;
    (t2848, t2856, t2859, t2860)
}
