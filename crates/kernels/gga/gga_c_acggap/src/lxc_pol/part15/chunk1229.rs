//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1229/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1229<F: Float>(t30730: F, t34767: F, t34771: F, t34783: F, t34802: F, t34804: F, t37249: F, t39485: F, t39489: F, t39494: F, t39497: F, t39502: F, t39506: F, t39508: F, t39511: F, t39514: F, t39517: F, t39520: F) -> F {
    let t41682 = -F::new(0.83861579438944405517e-3) * t34767 - F::new(0.16772315887788881104e-2) * t34771 - F::new(0.83861579438944405517e-3) * t34783 + t37249 + F::new(0.20965394859736101379e-2) * t34802 + F::new(0.41930789719472202758e-2) * t34804 - F::new(0.25724410870841842183e-1) * t39485 - F::new(0.12862205435420921092e-1) * t39489 + F::new(0.94344276868812456206e-2) * t39494 - F::new(0.62896184579208304138e-3) * t30730 - F::new(0.13719685797782315831e-1) * t39497 - F::new(0.31448092289604152069e-3) * t39502 + F::new(0.42874018118069736972e-3) * t39506 + F::new(0.28015625e-1) * t39508 + t39511 / F::new(12.0) + t39514 / F::new(24.0) + t39517 / F::new(12.0) - t39520 / F::new(12.0);
    t41682
}
