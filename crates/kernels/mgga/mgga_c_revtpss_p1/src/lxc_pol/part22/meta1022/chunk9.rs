//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3570/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3570<F: Float>(t20400: F, t3543: F, t1765: F, t57861: F, t16784: F, t5207: F, t12248: F, t3385: F, t6439: F, t3367: F, t60717: F, t1120: F, t128: F) -> (F, F, F, F, F, F) {
    let t68243 = F::cast_from(0.17315859105681463759e2_f64) * t20400 * t3543;
    let t68245 = F::cast_from(0.11696447245269292414e1_f64) * t57861 * t1765;
    let t68247 = F::cast_from(0.69263436422725855034e2_f64) * t16784 * t5207;
    let t68250 = F::cast_from(24.0_f64) * t12248 * t6439 * t3385;
    let t68251 = t3367 * t60717;
    let t68253 = t128 * t1120 * t68251;
    (t68243, t68245, t68247, t68250, t68251, t68253)
}
