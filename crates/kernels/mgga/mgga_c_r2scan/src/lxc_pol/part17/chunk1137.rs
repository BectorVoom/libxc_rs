//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1137/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1137<F: Float>(t39825: F, t39827: F, t39830: F, t39832: F, t39835: F, t39854: F, t39886: F, t39894: F, t39899: F, t39903: F, t39911: F, t39920: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41574 = F::new(0.28565981518604370584e-1) * t39825;
    let t41575 = F::new(0.95219938395347901946e-2) * t39827;
    let t41576 = F::new(0.95219938395347901946e-2) * t39830;
    let t41577 = F::new(0.28565981518604370584e-1) * t39832;
    let t41578 = F::new(0.93149212406257582492e-1) * t39835;
    let t41584 = F::new(0.13869154784086829701e1) * t39854;
    let t41601 = F::new(0.19514881078765566037e-1) * t39886;
    let t41605 = F::new(0.93149212406257582492e-1) * t39894;
    let t41607 = F::new(0.93149212406257582492e-1) * t39899;
    let t41608 = F::new(0.27944763721877274748e0) * t39903;
    let t41611 = F::new(0.46230515946956099004e0) * t39911;
    let t41615 = F::new(0.28565981518604370584e-1) * t39920;
    (t41574, t41575, t41576, t41577, t41578, t41584, t41601, t41605, t41607, t41608, t41611, t41615)
}
