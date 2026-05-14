//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 724/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk724<F: Float>(t8344: F, t38187: F, t7219: F, t7223: F, t7227: F, t7236: F, t7241: F, t7253: F, t7257: F, t7261: F, t8026: F, t8347: F, t8353: F, t8359: F, t8363: F, t8369: F) -> (F, F, F, F, F, F) {
    let t38188 = 0.72042316457491791906e-3 * t8344;
    let t38189 = t7219 + t7223 + t7227 + t7236 - t7241 + t8026 - t7253 - t7257 - t7261 + t38187 - t38188;
    let t38191 = 0.72042316457491791906e-3 * t8347;
    let t38192 = 0.72042316457491791906e-3 * t8353;
    let t38193 = 0.72042316457491791906e-3 * t8359;
    let t38194 = 0.72042316457491791906e-3 * t8363;
    let t38196 = 0.68186654135613354322e-2 * t8369;
    (t38189, t38191, t38192, t38193, t38194, t38196)
}
