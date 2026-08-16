//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1137/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1137(t39825: f64, t39827: f64, t39830: f64, t39832: f64, t39835: f64, t39854: f64, t39886: f64, t39894: f64, t39899: f64, t39903: f64, t39911: f64, t39920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41574 = 0.28565981518604370584e-1_f64 * t39825;
    let t41575 = 0.95219938395347901946e-2_f64 * t39827;
    let t41576 = 0.95219938395347901946e-2_f64 * t39830;
    let t41577 = 0.28565981518604370584e-1_f64 * t39832;
    let t41578 = 0.93149212406257582492e-1_f64 * t39835;
    let t41584 = 0.13869154784086829701e1_f64 * t39854;
    let t41601 = 0.19514881078765566037e-1_f64 * t39886;
    let t41605 = 0.93149212406257582492e-1_f64 * t39894;
    let t41607 = 0.93149212406257582492e-1_f64 * t39899;
    let t41608 = 0.27944763721877274748e0_f64 * t39903;
    let t41611 = 0.46230515946956099004e0_f64 * t39911;
    let t41615 = 0.28565981518604370584e-1_f64 * t39920;
    (t41574, t41575, t41576, t41577, t41578, t41584, t41601, t41605, t41607, t41608, t41611, t41615)
}
