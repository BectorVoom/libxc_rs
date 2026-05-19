//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 916/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk916<F: Float>(t1686: F, t2046: F, t2050: F, t31: F, t333: F, t3351: F, t511: F, t7248: F, t9216: F, t352: F, t515: F, t35407: F, t35413: F, t35424: F, t39771: F, t39773: F, t39777: F, t39781: F, t39786: F, t39789: F, t39792: F, t39797: F, t39801: F, t39804: F, t623: F, t7668: F) -> F {
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = F::cast_from(0.43368970657079495312e-4_f64) * t39808;
    let t39813 = t3351 * t7248 * t511 * t9216 * t333;
    let t39818 = t3351 * t7248 * t515 * t9216 * t352;
    let t39825 = F::cast_from(0.12769379967989351819e-4_f64) * t39771 + F::cast_from(0.17025839957319135759e-4_f64) * t39773 + F::cast_from(0.17025839957319135759e-4_f64) * t39777 + F::cast_from(0.85129199786595678796e-5_f64) * t39781 - t39786 - F::cast_from(0.15243824895787514157e-3_f64) * t39789 - F::cast_from(0.1951603679568577289e-3_f64) * t39792 - t39797 - t39801 - F::cast_from(0.15243824895787514157e-3_f64) * t39804 + t39809 + F::cast_from(0.76616279807936110914e-4_f64) * t39813 + F::cast_from(0.25538759935978703638e-4_f64) * t39818 - F::cast_from(0.47896966807455234256e0_f64) * t35407 - F::cast_from(0.15965655602485078085e0_f64) * t35413 - F::cast_from(0.19957069503106347607e-1_f64) * t623 * t7668 - F::cast_from(0.18183107769496894486e0_f64) * t35424;
    t39825
}
