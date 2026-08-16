//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 996/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk996(t7930: f64, t6090: f64, t6093: f64, t6156: f64, t7947: f64, t7955: f64, t834: f64, t841: f64, t2203: f64, t3046: f64, t836: f64, t6161: f64, t6180: f64, t6183: f64, t7931: f64, t7950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7957 = 2.0_f64 / 3.0_f64 * t7930;
    let t7958 = -t6156 + 8.0_f64 / 9.0_f64 * t6090 - t6093 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t7955 - t7957 + t7947;
    let t7959 = t834 * t7958;
    let t7961 = t841 * t7958;
    let t7966 = t2203 * t3046;
    let t7967 = t7966 * t836;
    let t7969 = -t6161 + 0.79724444444444444446e0_f64 * t6090 - 0.29896666666666666667e0_f64 * t6093 - t7931 + 0.8969e0_f64 * t7947 + 0.27385555555555555555e0_f64 * t7950 + 0.1898925e1_f64 * t7959 + 0.3071625e0_f64 * t7961 - 0.16431333333333333333e0_f64 * t6180 - 0.16431333333333333333e0_f64 * t6183 + 0.39862222222222222223e0_f64 * t7955 - 0.1898925e1_f64 * t7967;
    (t7958, t7959, t7961, t7966, t7967, t7969)
}
