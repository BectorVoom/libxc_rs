//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1232/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1232<F: Float>(t16502: F, t16504: F, t16506: F, t16508: F, t135: F, t1535: F, t16513: F, t16517: F, t20592: F, t23924: F, t23925: F, t2714: F, t2718: F, t3401: F, t5196: F, t6853: F) -> (F, F, F, F, F) {
    let t23929 = 8.0 * t16502;
    let t23933 = 24.0 * t16504;
    let t23934 = 48.0 * t16506;
    let t23935 = 96.0 * t16508;
    let t23939 = 6.0 * t135 * t3401 * t5196 + 6.0 * t1535 * t2714 * t6853 + 24.0 * t20592 * t2714 * t2718 - t16513 + t16517 - t23924 - t23925 - t23929 - t23933 + t23934 + t23935;
    (t23929, t23933, t23934, t23935, t23939)
}
