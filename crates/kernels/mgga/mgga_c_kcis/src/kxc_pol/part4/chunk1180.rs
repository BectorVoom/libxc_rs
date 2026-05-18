//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1180/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1180<F: Float>(t1071: F, t1745: F, t1154: F, t2630: F, t13480: F, t5134: F, t119: F, t41: F, t85: F, t13511: F, t5142: F, t1018: F, t1083: F) -> (F, F, F, F, F) {
    let t14999 = t1745 * t1071;
    let t15001 = t1154 * t14999 * t2630;
    let t15004 = t5134 * t13480;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15009 = t5142 * t13511;
    let t15012 = t1018 * t1083;
    (t15001, t15004, t15008, t15009, t15012)
}
