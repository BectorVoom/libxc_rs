//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 624/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk624<F: Float>(t1495: F, t3722: F, t1395: F, t1464: F, t2820: F, t509: F, t86: F) -> (F, F, F, F) {
    let t3723 = t1495 * t3722;
    let t3724 = t1395 * t3723;
    let t3725 = t1464 * t3724;
    let t3728 = t86 * t2820 * t509;
    (t3723, t3724, t3725, t3728)
}
