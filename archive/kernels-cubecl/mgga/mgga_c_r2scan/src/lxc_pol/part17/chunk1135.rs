//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1135/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1135<F: Float>(t39358: F, t39361: F, t39395: F, t39400: F, t39410: F, t39437: F, t39440: F, t39443: F, t39445: F, t39458: F, t39499: F, t39502: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41353 = F::cast_from(0.11426392607441748234e0_f64) * t39358;
    let t41354 = F::cast_from(0.46230515946956099004e0_f64) * t39361;
    let t41367 = F::cast_from(0.25610080155860322884e0_f64) * t39395;
    let t41369 = F::cast_from(0.13869154784086829701e1_f64) * t39400;
    let t41372 = F::cast_from(0.95219938395347901946e-2_f64) * t39410;
    let t41384 = F::cast_from(0.95219938395347901946e-2_f64) * t39437;
    let t41385 = F::cast_from(0.19043987679069580389e-1_f64) * t39440;
    let t41386 = F::cast_from(0.28565981518604370584e-1_f64) * t39443;
    let t41387 = F::cast_from(0.95219938395347901946e-2_f64) * t39445;
    let t41392 = F::cast_from(0.13869154784086829701e1_f64) * t39458;
    let t41414 = F::cast_from(0.46230515946956099004e0_f64) * t39499;
    let t41415 = F::cast_from(0.1536604809351619373e1_f64) * t39502;
    (t41353, t41354, t41367, t41369, t41372, t41384, t41385, t41386, t41387, t41392, t41414, t41415)
}
