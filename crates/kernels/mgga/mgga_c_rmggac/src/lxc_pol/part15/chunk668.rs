//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 668/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk668<F: Float>(t8543: F, t8546: F, t8549: F, t8552: F, t7438: F, t8612: F, t8617: F, t8655: F, t8669: F, t8677: F, t8822: F, t8710: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9335 = F::new(0.5987120850931904282e-1) * t8543;
    let t9336 = F::new(0.17961362552795712846e0) * t8546;
    let t9337 = F::new(0.35922725105591425692e0) * t8549;
    let t9338 = F::new(0.8980681276397856423e-1) * t8552;
    let t9339 = F::new(0.59590439850616975158e-4) * t7438;
    let t9368 = F::new(0.1064114997332445985e-4) * t8612;
    let t9369 = F::new(0.1064114997332445985e-4) * t8617;
    let t9381 = F::new(0.2993560425465952141e-1) * t8655;
    let t9393 = F::new(0.1064114997332445985e-4) * t8669;
    let t9412 = F::new(0.1064114997332445985e-4) * t8677;
    let t9440 = F::new(0.2993560425465952141e-1) * t8822;
    let t9445 = F::new(0.4838420607177634088e-3) * t8710;
    (t9335, t9336, t9337, t9338, t9339, t9368, t9369, t9381, t9393, t9412, t9440, t9445)
}
