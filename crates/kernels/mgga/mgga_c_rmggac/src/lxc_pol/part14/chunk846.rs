//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 846/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk846<F: Float>(t4928: F, t665: F, t2060: F, t5249: F, t739: F, t270: F, t574: F, t290: F, t2010: F, t7755: F, t1664: F, t7556: F) -> (F, F, F, F, F, F) {
    let t38809 = t665 * t4928;
    let t38812 = t2060 * t5249;
    let t38813 = t739 * t38812;
    let t38815 = t574 * t270;
    let t38816 = t290 * t38815;
    let t38818 = t2010 * t7755 * t38816;
    let t38819 = F::new(0.72042316457491791906e-3) * t38818;
    let t38820 = t1664 * t7556;
    (t38809, t38812, t38813, t38815, t38819, t38820)
}
