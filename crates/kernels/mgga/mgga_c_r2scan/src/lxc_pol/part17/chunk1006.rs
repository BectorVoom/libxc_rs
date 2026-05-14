//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1006/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1006<F: Float>(t39361: F, t39395: F, t39400: F, t39410: F, t39437: F, t39440: F, t39443: F, t39445: F, t39458: F, t39499: F, t39502: F, t39511: F, t39522: F, t39548: F, t39601: F, t39607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41354 = 0.46230515946956099004e0 * t39361;
    let t41367 = 0.25610080155860322884e0 * t39395;
    let t41369 = 0.13869154784086829701e1 * t39400;
    let t41372 = 0.95219938395347901946e-2 * t39410;
    let t41384 = 0.95219938395347901946e-2 * t39437;
    let t41385 = 0.19043987679069580389e-1 * t39440;
    let t41386 = 0.28565981518604370584e-1 * t39443;
    let t41387 = 0.95219938395347901946e-2 * t39445;
    let t41392 = 0.13869154784086829701e1 * t39458;
    let t41414 = 0.46230515946956099004e0 * t39499;
    let t41415 = 0.1536604809351619373e1 * t39502;
    let t41419 = 0.25610080155860322884e0 * t39511;
    let t41423 = 0.46230515946956099004e0 * t39522;
    let t41435 = 0.95219938395347901946e-2 * t39548;
    let t41464 = 0.10975748638225852664e-1 * t39601;
    let t41466 = 0.93149212406257582492e-1 * t39607;
    (t41354, t41367, t41369, t41372, t41384, t41385, t41386, t41387, t41392, t41414, t41415, t41419, t41423, t41435, t41464, t41466)
}
