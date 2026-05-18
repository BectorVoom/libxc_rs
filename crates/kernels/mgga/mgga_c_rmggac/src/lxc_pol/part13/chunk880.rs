//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 880/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk880<F: Float>(t5267: F, t7778: F, t903: F, t26144: F, t5181: F, t645: F, t27326: F, t7577: F, t5898: F, t2060: F, t27136: F, t30080: F, t8410: F) -> (F, F, F, F, F, F) {
    let t39535 = t903 * t7778 * t5267;
    let t39538 = t26144 * t645 * t5181;
    let t39541 = t903 * t7577 * t27326;
    let t39544 = t903 * t7778 * t5898;
    let t39547 = t903 * t2060 * t27136;
    let t39549 = t30080 * t8410;
    (t39535, t39538, t39541, t39544, t39547, t39549)
}
