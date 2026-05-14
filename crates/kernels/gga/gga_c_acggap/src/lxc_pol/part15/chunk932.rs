//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 932/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk932<F: Float>(t34390: F, t34398: F, t34409: F, t34421: F, t34429: F, t34488: F, t34500: F, t34506: F, t34512: F, t34534: F, t34537: F, t34556: F, t34570: F, t34592: F, t34609: F, t34618: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37065 = 0.1120625e0 * t34390;
    let t37069 = 0.11321313224257494745e-1 * t34398;
    let t37076 = 0.42874018118069736972e-3 * t34409;
    let t37087 = 7.0 / 72.0 * t34421;
    let t37090 = 0.21437009059034868486e-2 * t34429;
    let t37121 = 0.916875e-1 * t34488;
    let t37126 = 0.68598428988911579156e-2 * t34500;
    let t37129 = 0.34299214494455789578e-2 * t34506;
    let t37132 = 0.32012600194825403606e-1 * t34512;
    let t37140 = 0.34299214494455789578e-2 * t34534;
    let t37142 = 0.17149607247227894789e-2 * t34537;
    let t37150 = 0.12579236915841660828e-2 * t34556;
    let t37158 = 0.12862205435420921092e-1 * t34570;
    let t37167 = 11.0 / 96.0 * t34592;
    let t37175 = 11.0 / 96.0 * t34609;
    let t37179 = 0.2264262644851498949e-1 * t34618;
    (t37065, t37069, t37076, t37087, t37090, t37121, t37126, t37129, t37132, t37140, t37142, t37150, t37158, t37167, t37175, t37179)
}
