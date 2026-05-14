//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 866/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk866<F: Float>(t1550: F, t27102: F, t7577: F, t7778: F, t8377: F, t1632: F, t2064: F, t3928: F, t2373: F, t7561: F, t2283: F, t7944: F, t40965: F, t8620: F, t22: F, t235: F, t34812: F) -> (F, F, F, F, F, F, F) {
    let t41719 = t1550 * t7577 * t27102;
    let t41722 = t1550 * t7778 * t8377;
    let t41725 = t3928 * t2064 * t1632;
    let t41727 = t2373 * t7561;
    let t41730 = t7944 * t2283;
    let t41735 = t8620 * t40965;
    let t41738 = t235 * t34812 * t22;
    (t41719, t41722, t41725, t41727, t41730, t41735, t41738)
}
