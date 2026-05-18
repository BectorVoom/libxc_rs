//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 844/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk844<F: Float>(t7244: F, t8422: F, t2310: F, t7939: F, t2283: F, t38354: F, t7473: F, t118: F, t2281: F, t498: F, t7418: F, t9153: F) -> (F, F, F, F, F, F) {
    let t41828 = t7244 * t8422;
    let t41882 = t7939 * t2310;
    let t41884 = t7939 * t2283;
    let t41890 = t38354 * t7473;
    let t41914 = t7418 * t118 * t2281 * t498;
    let t41922 = t7244 * t9153;
    (t41828, t41882, t41884, t41890, t41914, t41922)
}
