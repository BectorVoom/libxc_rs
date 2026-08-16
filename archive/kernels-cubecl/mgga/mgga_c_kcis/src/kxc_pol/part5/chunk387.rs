//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 387/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk387<F: Float>(t1457: F, t509: F, t552: F, t557: F, t303: F, t1017: F, t86: F) -> (F, F, F, F) {
    let t1458 = t509 * t1457;
    let t1459 = t1458 * t552;
    let t1460 = t1459 * t557;
    let t1461 = t303 * t1460;
    let t1464 = t86 * t1017 * t509;
    (t1459, t1460, t1461, t1464)
}
