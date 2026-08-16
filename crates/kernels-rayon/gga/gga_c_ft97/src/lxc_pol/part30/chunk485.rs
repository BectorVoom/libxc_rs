//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 485/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk485(t295: f64, t312: f64, t7662: f64, t1501: f64, t6353: f64, t296: f64) -> (f64, f64, f64, f64) {
    let t7664 = t295 * t7662 * t312;
    let t7668 = t6353 * t1501;
    let t7669 = t296 * t7668;
    let t7672 = t1501 * t1501;
    (t7664, t7668, t7669, t7672)
}
