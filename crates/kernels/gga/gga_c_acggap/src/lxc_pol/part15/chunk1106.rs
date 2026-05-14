//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1106/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1106<F: Float>(t35985: F, t35991: F, t35995: F, t36004: F, t36022: F, t36032: F, t36036: F, t37819: F, t37822: F, t37826: F, t37827: F, t37830: F, t37833: F, t37836: F, t40398: F, t40403: F, t40408: F) -> (F,) {
    let t42089 = -0.25724410870841842183e-1 * t40398 + 0.28582678745379824648e-2 * t35985 + t37819 + 0.41930789719472202758e-2 * t35991 - 0.62896184579208304137e-2 * t35995 - t37822 - 0.20965394859736101379e-2 * t36004 - t37826 - t37827 - 0.18868855373762491242e-2 * t40403 - t37830 + 0.12579236915841660828e-2 * t36022 - 0.12862205435420921092e-2 * t40408 - t37833 + 0.264875e0 * t36032 + 0.305625e-1 * t36036 - t37836;
    (t42089,)
}
