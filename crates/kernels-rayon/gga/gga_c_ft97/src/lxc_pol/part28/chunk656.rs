//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 656/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk656(t376: f64, t6526: f64, t89: f64, t1307: f64, t3291: f64, t452: f64, t1901: f64, t23239: f64, t23263: f64, t23283: f64, t26412: f64, t26416: f64, t26420: f64, t26425: f64, t26428: f64, t26432: f64, t26437: f64, t26442: f64, t26446: f64, t446: f64) -> f64 {
    let t26451 = t89 * t376 * t6526;
    let t26454 = t452 * t3291 * t1307;
    let t26457 = -t23239 / 27.0_f64 + t446 * t26412 / 3.0_f64 + t446 * t26416 / 3.0_f64 + t446 * t26420 / 3.0_f64 + t446 * t26425 / 3.0_f64 + t26428 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t23263 - t446 * t26432 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t26437 + 2.0_f64 / 27.0_f64 * t1901 * t26442 - t1901 * t26446 / 9.0_f64 + t23283 / 9.0_f64 - t26451 / 9.0_f64 - t446 * t26454 / 3.0_f64;
    t26457
}
