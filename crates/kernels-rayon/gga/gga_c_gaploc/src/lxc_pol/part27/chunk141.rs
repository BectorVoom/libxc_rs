//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 141/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk141(t492: f64, t494: f64, t105: f64, t174: f64, t337: f64, t359: f64, t364: f64, t377: f64, t380: f64, t419: f64, t449: f64, t478: f64, t484: f64, t489: f64) -> (f64, f64) {
    let t495 = t492 * t494;
    let t498 = t337 + t359 - t364 - t377 + 0.37940008847568199465e-1_f64 * t380 * t174 + 0.28455006635676149599e-1_f64 * t419 * t174 - 0.28455006635676149599e-1_f64 * t105 * t449 + 0.28455006635676149599e-1_f64 * t105 * t478 - 0.31616674039640166221e-2_f64 * t484 * t489 - 0.28455006635676149599e-1_f64 * t105 * t495;
    (t495, t498)
}
