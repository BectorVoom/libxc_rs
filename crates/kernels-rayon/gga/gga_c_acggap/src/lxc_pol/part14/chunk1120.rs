//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1120/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1120(t142: f64, t6289: f64, t7436: f64, t6300: f64, t6309: f64, t30730: f64, t34767: f64, t34771: f64, t34783: f64, t34795: f64, t34803: f64, t37252: f64, t39485: f64, t39489: f64, t39494: f64, t39497: f64, t39502: f64, t39506: f64, t39508: f64, t39511: f64) -> f64 {
    let t39514 = t7436 * t142 * t6289;
    let t39517 = t7436 * t142 * t6300;
    let t39520 = t7436 * t142 * t6309;
    let t39522 = -0.41930789719472202757e-3_f64 * t34767 - 0.83861579438944405514e-3_f64 * t34771 - 0.41930789719472202757e-3_f64 * t34783 + t34795 + t34803 + t37252 - 0.12862205435420921092e-1_f64 * t39485 - 0.64311027177104605458e-2_f64 * t39489 + 0.47172138434406228102e-2_f64 * t39494 - 0.31448092289604152068e-3_f64 * t30730 - 0.68598428988911579156e-2_f64 * t39497 - 0.15724046144802076034e-3_f64 * t39502 + 0.21437009059034868486e-3_f64 * t39506 + 0.140078125e-1_f64 * t39508 + t39511 / 24.0_f64 + t39514 / 48.0_f64 + t39517 / 24.0_f64 - t39520 / 24.0_f64;
    t39522
}
