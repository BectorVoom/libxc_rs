//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1610/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1610<F: Float>(t6016: F, t2723: F, t5977: F, t231: F, t5966: F, t10770: F, t10871: F, t14586: F, t14791: F, t14894: F, t1544: F, t1559: F, t18426: F, t18444: F, t18469: F, t18627: F, t23245: F, t2745: F, t2747: F, t40673: F, t4362: F, t4364: F, t4365: F, t5962: F, t6017: F, t6022: F, t6035: F, t76284: F, t76289: F, t76313: F, t76315: F, t76330: F, t76337: F, t76362: F, t76705: F) -> (F, F, F, F, F) {
    let t87394 = t6016 * t6016;
    let t87395 = t87394 * t2723;
    let t87399 = t5977 * t5977;
    let t87400 = t87399 * t231;
    let t87417 = t231 * t5966;
    let t87470 = -F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t4364 * t4365 * t23245 - F::cast_from(0.25724410870841842184e-1_f64) * t2745 * t10770 * t18426 * t87417 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t76284 * t6035 + F::cast_from(0.10289764348336736874e0_f64) * t2745 * t40673 * t76705 * t1559 - F::cast_from(0.25724410870841842184e-1_f64) * t2745 * t10770 * t18444 * t87417 + F::cast_from(0.10289764348336736873e-1_f64) * t2745 * t14791 * t1559 * t1544 * t6016 - F::cast_from(0.15246000842785598467e-3_f64) * t76313 - F::cast_from(0.48018900292238105408e-1_f64) * t76315 - F::cast_from(0.48018900292238105408e-1_f64) * t76330 + F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t4364 * t76289 * t14586 - F::cast_from(0.10289764348336736873e-1_f64) * t4362 * t2747 * t18627 * t6022 + F::cast_from(0.20579528696673473747e-1_f64) * t14894 * t2747 * t76284 * t10871 * t1544 + F::cast_from(0.51448821741683684366e-2_f64) * t2745 * t2747 * t18426 * t231 * t5962 - F::cast_from(0.48018900292238105408e-1_f64) * t76337 + F::cast_from(0.30492001685571196935e-3_f64) * t76362 - F::cast_from(0.12862205435420921092e-2_f64) * t2745 * t4364 * t18426 * t6017 + F::cast_from(0.51448821741683684368e-1_f64) * t4362 * t10770 * t18469 * t6022;
    (t87394, t87395, t87399, t87400, t87470)
}
