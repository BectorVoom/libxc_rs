//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 803/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk803<F: Float>(t34557: F, t34558: F, t7362: F, t7369: F, t7373: F, t7378: F, t7382: F, t9758: F, t9759: F, t9760: F, t9761: F, t8494: F) -> (F, F) {
    let t38230 = t34557 - t34558 - t7362 - t9758 + t9759 - t9760 + t9761 + t7369 - t7373 + t7378 - t7382;
    let t38234 = F::new(0.85129199786595678796e-5) * t8494;
    (t38230, t38234)
}
