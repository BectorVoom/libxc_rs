//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 582/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk582<F: Float>(t7617: F, t854: F, t305: F, t830: F, t2100: F, t7587: F, t2103: F, t7591: F, t22: F, t3851: F, t36: F, t794: F) -> (F, F, F, F, F, F) {
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7628 = F::new(0.48783947674259960818e-1) * t7627;
    let t7629 = t2100 * t7587;
    let t7631 = t2103 * t7591;
    let t7633 = t3851 * t22;
    let t7634 = t36 * t794;
    (t7625, t7628, t7629, t7631, t7633, t7634)
}
