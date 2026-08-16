//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 916/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk916(t1686: f64, t2046: f64, t2050: f64, t31: f64, t333: f64, t3351: f64, t511: f64, t7248: f64, t9216: f64, t352: f64, t515: f64, t35407: f64, t35413: f64, t35424: f64, t39771: f64, t39773: f64, t39777: f64, t39781: f64, t39786: f64, t39789: f64, t39792: f64, t39797: f64, t39801: f64, t39804: f64, t623: f64, t7668: f64) -> f64 {
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = 0.43368970657079495312e-4_f64 * t39808;
    let t39813 = t3351 * t7248 * t511 * t9216 * t333;
    let t39818 = t3351 * t7248 * t515 * t9216 * t352;
    let t39825 = 0.12769379967989351819e-4_f64 * t39771 + 0.17025839957319135759e-4_f64 * t39773 + 0.17025839957319135759e-4_f64 * t39777 + 0.85129199786595678796e-5_f64 * t39781 - t39786 - 0.15243824895787514157e-3_f64 * t39789 - 0.1951603679568577289e-3_f64 * t39792 - t39797 - t39801 - 0.15243824895787514157e-3_f64 * t39804 + t39809 + 0.76616279807936110914e-4_f64 * t39813 + 0.25538759935978703638e-4_f64 * t39818 - 0.47896966807455234256e0_f64 * t35407 - 0.15965655602485078085e0_f64 * t35413 - 0.19957069503106347607e-1_f64 * t623 * t7668 - 0.18183107769496894486e0_f64 * t35424;
    t39825
}
