//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 702/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk702<F: Float>(t1756: F, t36: F, t2079: F, t262: F, t5271: F, t9708: F, t5259: F, t9704: F, t10053: F, t3814: F, t645: F, t9908: F) -> (F, F, F, F, F, F) {
    let t10122 = t36 * t1756;
    let t10124 = t2079 * t262 * t10122;
    let t10135 = t5271 * t9708;
    let t10137 = t5259 * t9704;
    let t10141 = t3814 * t10053;
    let t10151 = t9908 * t645;
    (t10122, t10124, t10135, t10137, t10141, t10151)
}
