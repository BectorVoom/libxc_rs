//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1167/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1167<F: Float>(t424: F, t5717: F, t5439: F, t717: F, t1762: F, t224: F, t5960: F, t5222: F, t5376: F, t5375: F, t1731: F, t20: F, t5947: F, t4911: F, t726: F, t21234: F, t226: F, t5455: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22219 = t424 * t5717;
    let t22222 = t424 * t717 * t5439;
    let t22227 = 0.11558335953042377059e2 * t1762 * t5960 * t224 * t5439;
    let t22228 = t5376 * t5222;
    let t22229 = t5375 * t22228;
    let t22232 = t1731 * t20 * t5947;
    let t22233 = t5375 * t22232;
    let t22235 = t4911 * t726;
    let t22239 = 0.84214420165938905383e2 * t5455 * t226 * t21234;
    (t22219, t22222, t22227, t22228, t22229, t22232, t22233, t22235, t22239)
}
