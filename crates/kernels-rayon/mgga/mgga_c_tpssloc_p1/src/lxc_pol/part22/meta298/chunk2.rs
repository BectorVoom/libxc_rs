//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1462/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1462(t13969: f64, t4599: f64, t3039: f64, t3069: f64, t4669: f64) -> (f64, f64, f64) {
    let t13970 = t13969 * t4599;
    let t13972 = t3039 * t13970 / 2304.0_f64;
    let t13995 = t4669 * t3069;
    (t13970, t13972, t13995)
}
