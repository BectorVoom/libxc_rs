//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1136/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1136(t39511: f64, t39522: f64, t39548: f64, t39601: f64, t39607: f64, t39627: f64, t39629: f64, t39640: f64, t39762: f64, t39785: f64, t39792: f64, t39823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41419 = 0.25610080155860322884e0_f64 * t39511;
    let t41423 = 0.46230515946956099004e0_f64 * t39522;
    let t41435 = 0.95219938395347901946e-2_f64 * t39548;
    let t41464 = 0.10975748638225852664e-1_f64 * t39601;
    let t41466 = 0.93149212406257582492e-1_f64 * t39607;
    let t41474 = 0.46230515946956099004e0_f64 * t39627;
    let t41475 = 0.13869154784086829701e1_f64 * t39629;
    let t41479 = 0.65854491829355115984e-1_f64 * t39640;
    let t41542 = 0.13869154784086829701e1_f64 * t39762;
    let t41552 = 0.39029762157531132074e-1_f64 * t39785;
    let t41555 = 0.46230515946956099004e0_f64 * t39792;
    let t41573 = 0.95219938395347901946e-2_f64 * t39823;
    (t41419, t41423, t41435, t41464, t41466, t41474, t41475, t41479, t41542, t41552, t41555, t41573)
}
