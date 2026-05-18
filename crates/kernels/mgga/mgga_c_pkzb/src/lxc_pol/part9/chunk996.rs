//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 996/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk996<F: Float>(t7930: F, t6090: F, t6093: F, t6156: F, t7947: F, t7955: F, t834: F, t841: F, t2203: F, t3046: F, t836: F, t6161: F, t6180: F, t6183: F, t7931: F, t7950: F) -> (F, F, F, F, F, F) {
    let t7957 = F::new(2.0) / F::new(3.0) * t7930;
    let t7958 = -t6156 + F::new(8.0) / F::new(9.0) * t6090 - t6093 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t7955 - t7957 + t7947;
    let t7959 = t834 * t7958;
    let t7961 = t841 * t7958;
    let t7966 = t2203 * t3046;
    let t7967 = t7966 * t836;
    let t7969 = -t6161 + F::new(0.79724444444444444446e0) * t6090 - F::new(0.29896666666666666667e0) * t6093 - t7931 + F::new(0.8969e0) * t7947 + F::new(0.27385555555555555555e0) * t7950 + F::new(0.1898925e1) * t7959 + F::new(0.3071625e0) * t7961 - F::new(0.16431333333333333333e0) * t6180 - F::new(0.16431333333333333333e0) * t6183 + F::new(0.39862222222222222223e0) * t7955 - F::new(0.1898925e1) * t7967;
    (t7958, t7959, t7961, t7966, t7967, t7969)
}
