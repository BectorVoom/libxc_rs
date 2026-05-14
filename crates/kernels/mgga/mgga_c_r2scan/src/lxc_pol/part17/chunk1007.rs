//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1007/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1007<F: Float>(t39627: F, t39629: F, t39640: F, t39762: F, t39785: F, t39792: F, t39823: F, t39825: F, t39827: F, t39830: F, t39832: F, t39835: F, t39854: F, t39886: F, t39894: F, t39899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41474 = 0.46230515946956099004e0 * t39627;
    let t41475 = 0.13869154784086829701e1 * t39629;
    let t41479 = 0.65854491829355115984e-1 * t39640;
    let t41542 = 0.13869154784086829701e1 * t39762;
    let t41552 = 0.39029762157531132074e-1 * t39785;
    let t41555 = 0.46230515946956099004e0 * t39792;
    let t41573 = 0.95219938395347901946e-2 * t39823;
    let t41574 = 0.28565981518604370584e-1 * t39825;
    let t41575 = 0.95219938395347901946e-2 * t39827;
    let t41576 = 0.95219938395347901946e-2 * t39830;
    let t41577 = 0.28565981518604370584e-1 * t39832;
    let t41578 = 0.93149212406257582492e-1 * t39835;
    let t41584 = 0.13869154784086829701e1 * t39854;
    let t41601 = 0.19514881078765566037e-1 * t39886;
    let t41605 = 0.93149212406257582492e-1 * t39894;
    let t41607 = 0.93149212406257582492e-1 * t39899;
    (t41474, t41475, t41479, t41542, t41552, t41555, t41573, t41574, t41575, t41576, t41577, t41578, t41584, t41601, t41605, t41607)
}
