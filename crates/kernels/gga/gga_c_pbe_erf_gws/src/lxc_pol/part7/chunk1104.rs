//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1104/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1104<F: Float>(t19756: F, t2417: F, t353: F, t4386: F, t19615: F, t814: F, t859: F, t19608: F, t19714: F, t19722: F, t19726: F, t19728: F, t19731: F, t19735: F, t19738: F, t19745: F, t19751: F, t2118: F, t2362: F, t2382: F, t2388: F, t2397: F, t3074: F, t3079: F, t328: F, t4395: F, t6112: F, t6135: F, t6158: F, t6793: F, t6802: F, t822: F, t833: F) -> F {
    let t19759 = t4386 * t353 * t19756 * t2417;
    let t19764 = t859 * t353 * t19615 * t814;
    let t19767 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t3074 * t2118 * t19714 * t328 * t3079 + t6802 * t2397 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19722 - t2388 * t6135 / F::cast_from(4.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19726 - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t19728 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t19731 + F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t19735 + t822 * t19738 * t833 / F::cast_from(96.0_f64) + t6112 * t2397 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t3074 * t4395 * t19745 * t2362 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t2382 * t6158 * t19751 * t2362 - t6793 * t19759 / F::cast_from(2.0_f64) + t19608 * t19764 / F::cast_from(16.0_f64);
    t19767
}
