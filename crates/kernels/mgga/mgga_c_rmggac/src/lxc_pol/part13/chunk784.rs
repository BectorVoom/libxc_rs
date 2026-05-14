//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 784/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk784<F: Float>(t1243: F, t236: F, t615: F, t7230: F, t7231: F, t34847: F, t8831: F, t1550: F, t5144: F, t7778: F, t2060: F, t27177: F, t4044: F, t5267: F, t903: F, t26144: F, t5181: F, t645: F) -> (F, F, F, F, F, F) {
    let t39523 = t7230 * t7231 * t236 * t615 * t1243;
    let t39525 = t34847 * t8831;
    let t39528 = t1550 * t7778 * t5144;
    let t39531 = t4044 * t2060 * t27177;
    let t39535 = t903 * t7778 * t5267;
    let t39538 = t26144 * t645 * t5181;
    (t39523, t39525, t39528, t39531, t39535, t39538)
}
