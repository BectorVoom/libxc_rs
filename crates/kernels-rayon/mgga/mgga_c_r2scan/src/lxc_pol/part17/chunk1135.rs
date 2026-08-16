//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1135/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1135(t39358: f64, t39361: f64, t39395: f64, t39400: f64, t39410: f64, t39437: f64, t39440: f64, t39443: f64, t39445: f64, t39458: f64, t39499: f64, t39502: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41353 = 0.11426392607441748234e0_f64 * t39358;
    let t41354 = 0.46230515946956099004e0_f64 * t39361;
    let t41367 = 0.25610080155860322884e0_f64 * t39395;
    let t41369 = 0.13869154784086829701e1_f64 * t39400;
    let t41372 = 0.95219938395347901946e-2_f64 * t39410;
    let t41384 = 0.95219938395347901946e-2_f64 * t39437;
    let t41385 = 0.19043987679069580389e-1_f64 * t39440;
    let t41386 = 0.28565981518604370584e-1_f64 * t39443;
    let t41387 = 0.95219938395347901946e-2_f64 * t39445;
    let t41392 = 0.13869154784086829701e1_f64 * t39458;
    let t41414 = 0.46230515946956099004e0_f64 * t39499;
    let t41415 = 0.1536604809351619373e1_f64 * t39502;
    (t41353, t41354, t41367, t41369, t41372, t41384, t41385, t41386, t41387, t41392, t41414, t41415)
}
