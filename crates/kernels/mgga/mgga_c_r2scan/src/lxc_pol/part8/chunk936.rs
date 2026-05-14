//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 936/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk936<F: Float>(t2667: F, t2727: F, t2177: F, t3198: F, t1632: F, t3190: F, t551: F, t2184: F, t2892: F, t2196: F, t3158: F, t378: F, t5: F, t5202: F, t5205: F, t5209: F, t5212: F, t5213: F, t5218: F, t5220: F, t5225: F, t5230: F, t5233: F, t5237: F) -> (F, F, F, F, F, F, F, F) {
    let t8861 = t2667 * t2727;
    let t8863 = t2177 * t3198;
    let t8865 = t1632 * t3190;
    let t8866 = t551 * t8865;
    let t8867 = t2184 * t8866;
    let t8872 = t1632 * t2892;
    let t8873 = t551 * t8872;
    let t8874 = t2196 * t8873;
    let t8879 = t5 * t378 * t3158;
    let t8884 = -t5202 - t5205 - t5209 + t5212 + 0.26680286006577777776e-2 * t5213 - t5218 - 0.33872559466666666666e-2 * t5220 - t5225 + t5230 - t5233 - 0.10843581300301739842e-1 * t5237;
    (t8861, t8863, t8866, t8867, t8873, t8874, t8879, t8884)
}
