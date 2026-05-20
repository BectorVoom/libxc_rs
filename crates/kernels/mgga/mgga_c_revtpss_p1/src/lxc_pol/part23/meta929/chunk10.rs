//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3043/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3043<F: Float>(t76396: F, t1733: F, t68947: F, t20629: F, t5105: F, t16835: F, t6471: F, t20448: F, t5063: F, t58466: F, t6474: F, t24262: F, t44101: F) -> (F, F, F, F, F, F, F) {
    let t81123 = -t76396;
    let t81128 = F::new(3.0) * t68947 * t1733;
    let t81130 = F::new(3.0) * t20629 * t5105;
    let t81132 = F::new(3.0) * t16835 * t6471;
    let t81134 = F::new(3.0) * t5063 * t20448;
    let t81136 = F::cast_from(0.48245938496077605201e2_f64) * t58466 * t6474;
    let t81138 = F::cast_from(0.96491876992155210402e2_f64) * t44101 * t24262;
    (t81123, t81128, t81130, t81132, t81134, t81136, t81138)
}
