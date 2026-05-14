//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 992/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk992<F: Float>(t2016: F, t9630: F, t1327: F, t507: F, t8888: F, t142: F, t6289: F, t7436: F, t6300: F, t6309: F, t30730: F, t34767: F, t34771: F, t34783: F, t34795: F, t34803: F, t37252: F, t39485: F, t39489: F, t39494: F, t39497: F, t39502: F, t39506: F) -> (F,) {
    let t39508 = t2016 * t9630;
    let t39511 = t8888 * t507 * t1327;
    let t39514 = t7436 * t142 * t6289;
    let t39517 = t7436 * t142 * t6300;
    let t39520 = t7436 * t142 * t6309;
    let t39522 = -0.41930789719472202757e-3 * t34767 - 0.83861579438944405514e-3 * t34771 - 0.41930789719472202757e-3 * t34783 + t34795 + t34803 + t37252 - 0.12862205435420921092e-1 * t39485 - 0.64311027177104605458e-2 * t39489 + 0.47172138434406228102e-2 * t39494 - 0.31448092289604152068e-3 * t30730 - 0.68598428988911579156e-2 * t39497 - 0.15724046144802076034e-3 * t39502 + 0.21437009059034868486e-3 * t39506 + 0.140078125e-1 * t39508 + t39511 / 24.0 + t39514 / 48.0 + t39517 / 24.0 - t39520 / 24.0;
    (t39522,)
}
