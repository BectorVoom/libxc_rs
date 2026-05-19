//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1048/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1048<F: Float>(t34390: F, t34398: F, t34409: F, t34421: F, t34429: F, t34488: F, t34500: F, t34506: F, t34512: F, t34534: F, t34537: F, t34556: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37065 = F::new(0.1120625e0) * t34390;
    let t37069 = F::cast_from(0.11321313224257494745e-1_f64) * t34398;
    let t37076 = F::cast_from(0.42874018118069736972e-3_f64) * t34409;
    let t37087 = F::new(7.0) / F::new(72.0) * t34421;
    let t37090 = F::cast_from(0.21437009059034868486e-2_f64) * t34429;
    let t37121 = F::new(0.916875e-1) * t34488;
    let t37126 = F::cast_from(0.68598428988911579156e-2_f64) * t34500;
    let t37129 = F::cast_from(0.34299214494455789578e-2_f64) * t34506;
    let t37132 = F::cast_from(0.32012600194825403606e-1_f64) * t34512;
    let t37140 = F::cast_from(0.34299214494455789578e-2_f64) * t34534;
    let t37142 = F::cast_from(0.17149607247227894789e-2_f64) * t34537;
    let t37150 = F::cast_from(0.12579236915841660828e-2_f64) * t34556;
    (t37065, t37069, t37076, t37087, t37090, t37121, t37126, t37129, t37132, t37140, t37142, t37150)
}
