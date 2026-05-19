//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1279/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1279<F: Float>(t11243: F, t18790: F, t11261: F, t2192: F, t11184: F, t18480: F, t836: F, t3046: F, t9771: F, t3747: F, t7966: F, t3041: F, t9798: F) -> (F, F, F, F, F, F) {
    let t31196 = F::cast_from(0.96491876992155210402e2_f64) * t18790 * t11243;
    let t31198 = F::new(1.0) * t2192 * t11261;
    let t31204 = t18480 * t11184 * t836;
    let t31206 = t9771 * t3046;
    let t31208 = t7966 * t3747;
    let t31210 = t3041 * t9798;
    (t31196, t31198, t31204, t31206, t31208, t31210)
}
