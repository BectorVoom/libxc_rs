//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 66/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk66(t122: f64, t158: f64, t169: f64, t172: f64, t105: f64, t33: f64, t58: f64) -> (f64, f64, f64) {
    let t174 = t122 * t158 * t169 * t172;
    let t177 = -t33 + t58 + 0.28455006635676149599e-1_f64 * t105 * t174;
    let t178 = f64::sqrt(4.0_f64);
    (t174, t177, t178)
}
