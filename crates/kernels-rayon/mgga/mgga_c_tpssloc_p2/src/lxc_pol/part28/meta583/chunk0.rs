//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1871/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871(t252: f64, t87230: f64, t13230: f64, t87052: f64, t23168: f64, t25321: f64, t25284: f64, t6579: f64, t13388: f64, t1888: f64, t6646: f64, t13385: f64, t22996: f64) -> (f64, f64, f64, f64, f64) {
    let t87529 = t87230 * t252;
    let t87531 = t87052 * t87529 * t13230;
    let t87533 = t23168 * t25321;
    let t87535 = t6579 * t25284;
    let t87538 = t1888 * t6646 * t13388;
    let t87541 = t1888 * t22996 * t13385;
    (t87531, t87533, t87535, t87538, t87541)
}
