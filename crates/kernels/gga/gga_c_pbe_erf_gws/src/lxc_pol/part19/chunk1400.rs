//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1400/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1400<F: Float>(t14918: F, t3083: F, t1115: F, t12201: F, t14311: F, t14327: F, t3917: F, t4083: F, t54882: F, t55962: F, t57687: F, t57689: F, t57694: F, t57696: F, t57700: F, t57702: F, t57705: F, t57707: F, t57711: F) -> F {
    let t58929 = t3083 * t14918;
    let t58940 = t57687 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t57689 - t57694 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t57696 - t12201 * t4083 / F::cast_from(96.0_f64) - t1115 * t54882 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t58929 - t3917 * t14311 / F::cast_from(96.0_f64) - t3917 * t14327 / F::cast_from(96.0_f64) - t57700 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t57702 - t57705 / F::cast_from(12.0_f64) - t55962 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57707 + t57711 / F::cast_from(384.0_f64);
    t58940
}
