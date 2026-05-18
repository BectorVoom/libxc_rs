//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1026/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1026<F: Float>(t8578: F, t8583: F, t8585: F, t8588: F, t8590: F, t8593: F, t8595: F, t8598: F, t8604: F, t8610: F, t8612: F, t8617: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42459 = F::new(0.1702583995731913576e-4) * t8578;
    let t42460 = F::new(0.1702583995731913576e-4) * t8583;
    let t42461 = F::new(0.5107751987195740728e-4) * t8585;
    let t42462 = F::new(0.5107751987195740728e-4) * t8588;
    let t42463 = F::new(0.5107751987195740728e-4) * t8590;
    let t42464 = F::new(0.5107751987195740728e-4) * t8593;
    let t42465 = F::new(0.1702583995731913576e-4) * t8595;
    let t42466 = F::new(0.1702583995731913576e-4) * t8598;
    let t42468 = F::new(0.1702583995731913576e-4) * t8604;
    let t42469 = F::new(0.1702583995731913576e-4) * t8610;
    let t42470 = F::new(0.212822999466489197e-4) * t8612;
    let t42471 = F::new(0.212822999466489197e-4) * t8617;
    (t42459, t42460, t42461, t42462, t42463, t42464, t42465, t42466, t42468, t42469, t42470, t42471)
}
