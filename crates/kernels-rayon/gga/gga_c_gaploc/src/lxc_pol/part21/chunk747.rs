//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 747/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk747(t6636: f64, t6692: f64, t6754: f64, t6814: f64, t6875: f64, t6931: f64, t6984: f64, t7055: f64, t481: f64, t686: f64, t941: f64) -> (f64, f64) {
    let t7058 = t6636 + t6692 + t6754 + t6814 + t6875 + t6931 + t6984 + t7055;
    let t7064 = t481 * t941 * t686;
    (t7058, t7064)
}
