//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1277/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1277<F: Float>(t1330: F, t21020: F, t26: F, t659: F, t6979: F, t6982: F, t21110: F, t542: F, t4620: F) -> (F, F, F, F) {
    let t21198 = t1330 * t21020;
    let t21199 = t26 * t21198;
    let t21201 = t659 * t6979;
    let t21203 = t659 * t6982;
    let t21205 = t542 * t21110;
    let t21206 = t4620 * t21205;
    (t21199, t21201, t21203, t21206)
}
