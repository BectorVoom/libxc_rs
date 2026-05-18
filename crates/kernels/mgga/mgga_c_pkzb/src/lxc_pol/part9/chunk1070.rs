//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1070/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1070<F: Float>(t16666: F, t16732: F, t16794: F, t16869: F, t83: F, t99: F, t501: F, t5076: F, t5169: F, t496: F, t5175: F, t4882: F, t546: F) -> (F, F, F, F, F) {
    let t16873 = t83 * t99 * (t16666 + t16732 + t16794 + t16869);
    let t16875 = F::new(16.0) * t501 * t5076;
    let t16876 = t501 * t5169;
    let t16878 = t496 * t5175;
    let t16880 = t4882 * t546;
    (t16873, t16875, t16876, t16878, t16880)
}
