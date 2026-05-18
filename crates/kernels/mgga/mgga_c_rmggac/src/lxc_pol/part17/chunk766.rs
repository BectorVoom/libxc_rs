//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 766/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk766<F: Float>(t326: F, t35928: F, t2078: F, t26: F, t3814: F, t36: F, t4616: F, t34805: F, t648: F, t305: F, t35590: F, t2115: F, t35876: F) -> (F, F, F, F, F, F, F) {
    let t35929 = t326 * t35928;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t35972 = t4616 * t36;
    let t36034 = t648 * t34805;
    let t36035 = F::new(0.15556658869458454171e0) * t36034;
    let t36058 = t305 * t35590;
    let t36088 = t2115 * t35876;
    (t35929, t35959, t35960, t35972, t36035, t36058, t36088)
}
