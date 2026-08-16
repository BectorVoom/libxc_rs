//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1170/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1170(t34382: f64, t34390: f64, t34392: f64, t34394: f64, t34396: f64, t34398: f64, t34400: f64, t34409: f64, t30463: f64, t30469: f64, t30497: f64, t30507: f64, t30511: f64, t30522: f64, t30524: f64, t34385: f64, t34388: f64, t34407: f64) -> f64 {
    let t37062 = 7.0_f64 / 72.0_f64 * t34382;
    let t37065 = 0.1120625e0_f64 * t34390;
    let t37066 = 0.26147916666666666667e0_f64 * t34392;
    let t37067 = 0.42874018118069736972e-3_f64 * t34394;
    let t37068 = 0.16006300097412701803e-1_f64 * t34396;
    let t37069 = 0.11321313224257494745e-1_f64 * t34398;
    let t37070 = 0.34299214494455789578e-2_f64 * t34400;
    let t37076 = 0.42874018118069736972e-3_f64 * t34409;
    let t37078 = -0.18868855373762491241e-2_f64 * t30463 + 0.68598428988911579156e-2_f64 * t30469 + t37062 - 5.0_f64 / 16.0_f64 * t34385 - t34388 / 32.0_f64 - t37065 + t37066 - t37067 + t37068 - t37069 - t37070 - 0.75475421495049964964e-2_f64 * t30497 + 0.11321313224257494745e-1_f64 * t30507 + 0.21437009059034868486e-3_f64 * t30511 - 0.18868855373762491241e-2_f64 * t30522 - 0.68598428988911579156e-1_f64 * t34407 - t37076 + 0.12862205435420921092e-2_f64 * t30524;
    t37078
}
