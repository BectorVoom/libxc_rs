//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 800/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk800<F: Float>(t8347: F, t8353: F, t8359: F, t8363: F, t8369: F, t8372: F, t8375: F, t8379: F, t8385: F, t8388: F, t8391: F, t8394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38191 = F::new(0.72042316457491791906e-3) * t8347;
    let t38192 = F::new(0.72042316457491791906e-3) * t8353;
    let t38193 = F::new(0.72042316457491791906e-3) * t8359;
    let t38194 = F::new(0.72042316457491791906e-3) * t8363;
    let t38196 = F::new(0.68186654135613354322e-2) * t8369;
    let t38197 = F::new(0.23948483403727617128e0) * t8372;
    let t38198 = F::new(0.35922725105591425692e0) * t8375;
    let t38200 = F::new(0.23948483403727617128e0) * t8379;
    let t38203 = F::new(0.23948483403727617128e0) * t8385;
    let t38204 = F::new(0.23948483403727617128e0) * t8388;
    let t38205 = F::new(0.23948483403727617128e0) * t8391;
    let t38206 = F::new(0.35922725105591425692e0) * t8394;
    (t38191, t38192, t38193, t38194, t38196, t38197, t38198, t38200, t38203, t38204, t38205, t38206)
}
