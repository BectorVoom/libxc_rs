//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1229/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1229(t30730: f64, t34767: f64, t34771: f64, t34783: f64, t34802: f64, t34804: f64, t37249: f64, t39485: f64, t39489: f64, t39494: f64, t39497: f64, t39502: f64, t39506: f64, t39508: f64, t39511: f64, t39514: f64, t39517: f64, t39520: f64) -> f64 {
    let t41682 = -0.83861579438944405517e-3_f64 * t34767 - 0.16772315887788881104e-2_f64 * t34771 - 0.83861579438944405517e-3_f64 * t34783 + t37249 + 0.20965394859736101379e-2_f64 * t34802 + 0.41930789719472202758e-2_f64 * t34804 - 0.25724410870841842183e-1_f64 * t39485 - 0.12862205435420921092e-1_f64 * t39489 + 0.94344276868812456206e-2_f64 * t39494 - 0.62896184579208304138e-3_f64 * t30730 - 0.13719685797782315831e-1_f64 * t39497 - 0.31448092289604152069e-3_f64 * t39502 + 0.42874018118069736972e-3_f64 * t39506 + 0.28015625e-1_f64 * t39508 + t39511 / 12.0_f64 + t39514 / 24.0_f64 + t39517 / 12.0_f64 - t39520 / 12.0_f64;
    t41682
}
