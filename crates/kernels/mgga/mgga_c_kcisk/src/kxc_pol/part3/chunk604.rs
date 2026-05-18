//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 604/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk604<F: Float>(t5139: F, t5171: F, t1689: F, t1809: F, t1860: F, t4794: F, t5084: F, t5085: F, t5087: F, t5089: F, t5090: F, t5094: F, t5097: F, t5102: F, t5105: F, t604: F, t674: F, t702: F) -> (F, F) {
    let t5172 = t5139 + t5171;
    let t5174 = t5084 + F::new(0.46853067927761790996e-2) * t5085 + F::new(0.93706135855523581992e-2) * t5087 + F::new(0.46853067927761790996e-2) * t5089 * t5090 + F::new(0.93706135855523581992e-2) * t1809 * t5094 - F::new(0.23426533963880895498e-2) * t1809 * t5097 + F::new(0.14055920378328537299e-1) * t674 * t5102 - F::new(0.46853067927761790996e-2) * t674 * t5105 - t4794 * t702 - F::new(2.0) * t1689 * t1860 - t604 * t5172;
    (t5172, t5174)
}
