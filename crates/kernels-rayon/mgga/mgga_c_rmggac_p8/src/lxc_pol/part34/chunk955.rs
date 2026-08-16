//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 955/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk955(t70885: f64, t74240: f64, t70892: f64, t74247: f64, t74249: f64, t74253: f64, t74256: f64, t15492: f64, t2019: f64, t2020: f64, t74259: f64, t74262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76935 = 0.99317399751028291929e-5_f64 * t70885;
    let t76937 = 0.38430329123504567781e-4_f64 * t74240;
    let t76939 = 0.15243824895787514157e-3_f64 * t70892;
    let t76940 = 0.85129199786595678799e-5_f64 * t74247;
    let t76941 = 0.85129199786595678799e-5_f64 * t74249;
    let t76942 = 0.85129199786595678799e-5_f64 * t74253;
    let t76943 = 0.72042316457491791901e-3_f64 * t74256;
    let t76945 = t2019 * t2020 * t15492;
    let t76946 = 0.15243824895787514157e-3_f64 * t76945;
    let t76947 = 0.30487649791575028312e-3_f64 * t74259;
    let t76948 = 0.72042316457491791901e-3_f64 * t74262;
    (t76935, t76937, t76939, t76940, t76941, t76942, t76943, t76946, t76947, t76948)
}
