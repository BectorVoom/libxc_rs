//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1170/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1170<F: Float>(t34382: F, t34390: F, t34392: F, t34394: F, t34396: F, t34398: F, t34400: F, t34409: F, t30463: F, t30469: F, t30497: F, t30507: F, t30511: F, t30522: F, t30524: F, t34385: F, t34388: F, t34407: F) -> F {
    let t37062 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t34382;
    let t37065 = F::cast_from(0.1120625e0_f64) * t34390;
    let t37066 = F::cast_from(0.26147916666666666667e0_f64) * t34392;
    let t37067 = F::cast_from(0.42874018118069736972e-3_f64) * t34394;
    let t37068 = F::cast_from(0.16006300097412701803e-1_f64) * t34396;
    let t37069 = F::cast_from(0.11321313224257494745e-1_f64) * t34398;
    let t37070 = F::cast_from(0.34299214494455789578e-2_f64) * t34400;
    let t37076 = F::cast_from(0.42874018118069736972e-3_f64) * t34409;
    let t37078 = -F::cast_from(0.18868855373762491241e-2_f64) * t30463 + F::cast_from(0.68598428988911579156e-2_f64) * t30469 + t37062 - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t34385 - t34388 / F::cast_from(32.0_f64) - t37065 + t37066 - t37067 + t37068 - t37069 - t37070 - F::cast_from(0.75475421495049964964e-2_f64) * t30497 + F::cast_from(0.11321313224257494745e-1_f64) * t30507 + F::cast_from(0.21437009059034868486e-3_f64) * t30511 - F::cast_from(0.18868855373762491241e-2_f64) * t30522 - F::cast_from(0.68598428988911579156e-1_f64) * t34407 - t37076 + F::cast_from(0.12862205435420921092e-2_f64) * t30524;
    t37078
}
