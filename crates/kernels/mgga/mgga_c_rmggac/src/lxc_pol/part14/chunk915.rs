//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 915/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk915<F: Float>(t26157: F, t5223: F, t645: F, t1635: F, t2064: F, t4044: F, t1550: F, t27102: F, t7577: F, t7778: F, t8377: F, t1632: F, t3928: F, t2373: F, t7561: F, t2283: F, t7944: F) -> (F, F, F, F, F, F, F) {
    let t41713 = t26157 * t645 * t5223;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = 0.95793933614910468512e0 * t41716;
    let t41719 = t1550 * t7577 * t27102;
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = 0.15965655602485078085e0 * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = 0.47896966807455234256e0 * t41725;
    let t41727 = t2373 * t7561;
    let t41730 = t7944 * t2283;
    (t41713, t41717, t41719, t41723, t41726, t41727, t41730)
}
