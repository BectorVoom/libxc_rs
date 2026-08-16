//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1390/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1390<F: Float>(t12007: F, t12089: F, t12103: F, t12110: F, t1402: F, t1429: F, t1508: F, t1599: F, t1641: F, t193: F, t30708: F, t30712: F, t34486: F, t34489: F, t34492: F, t34498: F, t34500: F, t34503: F, t34510: F, t34512: F, t3701: F, t3710: F, t4428: F, t4634: F) -> F {
    let t38588 = -t30708 - t34486 - t34489 - t34492 + F::cast_from(0.35750489951850426669e0_f64) * t1508 * t3701 * t193 - F::cast_from(0.23005755572352449806e1_f64) * t4634 * t3710 - F::cast_from(0.46011511144704899612e1_f64) * t1641 * t12110 - F::cast_from(0.71500979903700853338e0_f64) * t1599 * t12103 - t34498 - t34500 - t34503 + t34510 + t34512 + F::cast_from(0.1022478025437886658e1_f64) * t4428 * t12089 - t30712 - F::cast_from(0.92686455430723328401e-1_f64) * t1429 * t1402 * t12007;
    t38588
}
