//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 852/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk852<F: Float>(t6090: F, t6156: F, t7955: F, t7957: F, t9782: F, t9797: F, t834: F, t3743: F, t6165: F, t836: F, t3046: F, t3052: F, t6161: F, t7931: F, t9772: F, t9774: F, t9777: F) -> (F, F, F, F, F, F) {
    let t9798 = -t6156 + 4.0 / 9.0 * t6090 + 8.0 / 9.0 * t7955 - t7957 - t9782 / 3.0 + t9797;
    let t9799 = t834 * t9798;
    let t9805 = t6165 * t3743;
    let t9806 = t9805 * t836;
    let t9808 = t3052 * t3046;
    let t9810 = 0.142419375e1 * t9772 - 0.1898925e1 * t9774 - 0.9494625e0 * t9777 + 0.1898925e1 * t9799 - t6161 + 0.39862222222222222223e0 * t6090 + 0.79724444444444444445e0 * t7955 - t7931 - 0.29896666666666666667e0 * t9782 + 0.8969e0 * t9797 - 0.76790625e-1 * t9806 + 0.3071625e0 * t9808;
    (t9798, t9799, t9805, t9806, t9808, t9810)
}
