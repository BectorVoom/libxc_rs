//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1136/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1136<F: Float>(t39511: F, t39522: F, t39548: F, t39601: F, t39607: F, t39627: F, t39629: F, t39640: F, t39762: F, t39785: F, t39792: F, t39823: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41419 = F::new(0.25610080155860322884e0) * t39511;
    let t41423 = F::new(0.46230515946956099004e0) * t39522;
    let t41435 = F::new(0.95219938395347901946e-2) * t39548;
    let t41464 = F::new(0.10975748638225852664e-1) * t39601;
    let t41466 = F::new(0.93149212406257582492e-1) * t39607;
    let t41474 = F::new(0.46230515946956099004e0) * t39627;
    let t41475 = F::new(0.13869154784086829701e1) * t39629;
    let t41479 = F::new(0.65854491829355115984e-1) * t39640;
    let t41542 = F::new(0.13869154784086829701e1) * t39762;
    let t41552 = F::new(0.39029762157531132074e-1) * t39785;
    let t41555 = F::new(0.46230515946956099004e0) * t39792;
    let t41573 = F::new(0.95219938395347901946e-2) * t39823;
    (t41419, t41423, t41435, t41464, t41466, t41474, t41475, t41479, t41542, t41552, t41555, t41573)
}
