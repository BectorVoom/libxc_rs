//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2331/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2331(t28817: f64, t6876: f64, t1983: f64, t28826: f64, t83859: f64, t26149: f64, t7685: f64, t100828: f64, t100833: f64, t100835: f64, t100838: f64, t100840: f64, t1458: f64, t1459: f64, t19461: f64, t19534: f64, t1976: f64, t2314: f64, t24980: f64, t25958: f64, t28855: f64, t4026: f64, t4034: f64, t5107: f64, t5457: f64, t6468: f64, t652: f64, t6862: f64, t6872: f64, t7451: f64, t7458: f64, t7670: f64, t90400: f64) -> f64 {
    let t100854 = 6.0_f64 * t6876 * t28817;
    let t100861 = 6.0_f64 * t1983 * t83859 * t28826;
    let t100863 = 2.0_f64 * t7685 * t26149;
    let t100864 = -4.0_f64 * t1458 * t25958 * t652 - 2.0_f64 * t19534 * t1976 * t652 - 4.0_f64 * t1459 * t90400 - 2.0_f64 * t19461 * t1976 - 4.0_f64 * t2314 * t28855 - 4.0_f64 * t24980 * t7458 - 4.0_f64 * t28855 * t4034 - 2.0_f64 * t4026 * t7670 - 2.0_f64 * t5107 * t7451 - 2.0_f64 * t5457 * t6862 + t6468 * t6872 + t100828 - t100833 - t100835 + t100838 - t100840 + t100854 + t100861 - t100863;
    t100864
}
