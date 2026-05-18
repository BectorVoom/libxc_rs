//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 903/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk903<F: Float>(t6090: F, t6156: F, t7955: F, t7957: F, t9782: F, t9797: F, t834: F, t3743: F, t6165: F, t836: F, t3046: F, t3052: F) -> (F, F, F, F, F) {
    let t9798 = -t6156 + F::new(4.0) / F::new(9.0) * t6090 + F::new(8.0) / F::new(9.0) * t7955 - t7957 - t9782 / F::new(3.0) + t9797;
    let t9799 = t834 * t9798;
    let t9805 = t6165 * t3743;
    let t9806 = t9805 * t836;
    let t9808 = t3052 * t3046;
    (t9798, t9799, t9805, t9806, t9808)
}
