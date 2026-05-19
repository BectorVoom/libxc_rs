//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 844/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk844<F: Float>(t41667: F, t1635: F, t2064: F, t4044: F, t1550: F, t7778: F, t8377: F, t1632: F, t3928: F, t2373: F, t7561: F, t40965: F, t8620: F) -> (F, F, F, F, F, F) {
    let t41668 = F::cast_from(0.43368970657079495312e-4_f64) * t41667;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = F::cast_from(0.95793933614910468512e0_f64) * t41716;
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = F::cast_from(0.15965655602485078085e0_f64) * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = F::cast_from(0.47896966807455234256e0_f64) * t41725;
    let t41727 = t2373 * t7561;
    let t41735 = t8620 * t40965;
    (t41668, t41717, t41723, t41726, t41727, t41735)
}
