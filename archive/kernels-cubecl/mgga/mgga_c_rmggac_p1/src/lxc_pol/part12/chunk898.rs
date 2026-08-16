//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 898/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk898<F: Float>(t34847: F, t8831: F, t1550: F, t5144: F, t7778: F, t2060: F, t27177: F, t4044: F, t289: F, t35124: F, t35128: F, t35130: F, t35132: F, t39491: F, t39493: F, t39495: F, t39497: F, t39499: F, t39506: F, t39507: F, t39514: F, t39518: F, t39523: F) -> F {
    let t39525 = t34847 * t8831;
    let t39528 = t1550 * t7778 * t5144;
    let t39529 = F::cast_from(0.15965655602485078085e0_f64) * t39528;
    let t39531 = t4044 * t2060 * t27177;
    let t39533 = -F::cast_from(0.76616279807936110914e-4_f64) * t39491 - F::cast_from(0.25538759935978703638e-4_f64) * t39493 + F::cast_from(0.25538759935978703638e-4_f64) * t39495 + F::cast_from(0.85129199786595678796e-5_f64) * t39497 + F::cast_from(0.1064114997332445985e-4_f64) * t39499 - F::cast_from(0.15243824895787514157e-3_f64) * t35124 + F::cast_from(0.21684485328539747656e-4_f64) * t35128 - F::cast_from(0.90915538847484472429e-2_f64) * t35130 + F::cast_from(0.15965655602485078085e0_f64) * t35132 - t39506 - F::cast_from(0.4726e1_f64) * t289 * t39507 - F::cast_from(0.85129199786595678796e-5_f64) * t39514 - F::cast_from(0.85129199786595678796e-5_f64) * t39518 + F::cast_from(0.53205749866622299248e-5_f64) * t39523 - F::cast_from(0.31923449919973379548e-4_f64) * t39525 - t39529 + F::cast_from(0.17961362552795712846e0_f64) * t39531;
    t39533
}
