//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 785/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk785<F: Float>(t27326: F, t7577: F, t903: F, t5898: F, t7778: F, t2060: F, t27136: F, t30080: F, t8410: F, t290: F, t38843: F, t2012: F, t7349: F, t2412: F, t7424: F, t7421: F) -> (F, F, F, F, F, F, F) {
    let t39541 = t903 * t7577 * t27326;
    let t39544 = t903 * t7778 * t5898;
    let t39547 = t903 * t2060 * t27136;
    let t39549 = t30080 * t8410;
    let t39553 = t290 * t38843;
    let t39555 = t7349 * t2012 * t39553;
    let t39559 = t2412 * t7424;
    let t39561 = t2412 * t7421;
    (t39541, t39544, t39547, t39549, t39555, t39559, t39561)
}
