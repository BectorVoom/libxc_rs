//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1421/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1421(t12213: f64, t12223: f64, t1391: f64, t1392: f64, t2684: f64, t28249: f64, t28259: f64, t28281: f64, t32968: f64, t32972: f64, t32973: f64, t32974: f64, t32979: f64, t32984: f64, t32987: f64, t32991: f64, t32997: f64, t33001: f64, t33004: f64, t825: f64) -> f64 {
    let t38993 = 0.11360866949309851756e0_f64 * t2684 * t1391 * t1392 * t12213 - 0.11360866949309851756e0_f64 * t825 * t1391 * t1392 * t12223 - t32968 - t32972 - t32973 + t32974 - t28249 - t28259 - t32979 - t32984 - t32987 + t32991 + t32997 - t33001 + t33004 + t28281;
    t38993
}
