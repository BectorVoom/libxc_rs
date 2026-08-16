//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1260/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1260(t35985: f64, t35991: f64, t35995: f64, t36004: f64, t36022: f64, t36032: f64, t36036: f64, t37819: f64, t37822: f64, t37826: f64, t37827: f64, t37830: f64, t37833: f64, t37836: f64, t40398: f64, t40403: f64, t40408: f64) -> f64 {
    let t42089 = -0.25724410870841842183e-1_f64 * t40398 + 0.28582678745379824648e-2_f64 * t35985 + t37819 + 0.41930789719472202758e-2_f64 * t35991 - 0.62896184579208304137e-2_f64 * t35995 - t37822 - 0.20965394859736101379e-2_f64 * t36004 - t37826 - t37827 - 0.18868855373762491242e-2_f64 * t40403 - t37830 + 0.12579236915841660828e-2_f64 * t36022 - 0.12862205435420921092e-2_f64 * t40408 - t37833 + 0.264875e0_f64 * t36032 + 0.305625e-1_f64 * t36036 - t37836;
    t42089
}
