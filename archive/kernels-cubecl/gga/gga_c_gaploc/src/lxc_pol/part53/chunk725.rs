//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 725/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk725<F: Float>(t12452: F, t12456: F, t12924: F, t12928: F, t12929: F, t12930: F, t12935: F, t12936: F, t12937: F, t12941: F, t13780: F, t13783: F) -> F {
    let t14463 = t12924 - t12928 - t12929 + t12930 - F::cast_from(0.89376224879626066675e-1_f64) * t12452 + F::cast_from(0.59584149919750711115e-1_f64) * t12456 - F::cast_from(0.38342925953920749676e0_f64) * t13780 + F::cast_from(0.38342925953920749676e0_f64) * t13783 - t12935 + t12936 + t12937 - t12941;
    t14463
}
