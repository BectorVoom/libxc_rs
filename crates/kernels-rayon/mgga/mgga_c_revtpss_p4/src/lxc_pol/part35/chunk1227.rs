//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1227/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1227(t103586: f64, t110177: f64, t113096: f64, t113103: f64, t113107: f64, t113432: f64, t113440: f64, t115747: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2070: f64, t2071: f64, t23114: f64, t23148: f64, t23279: f64, t23421: f64, t23429: f64, t2403: f64, t26590: f64, t28460: f64, t29598: f64, t30420: f64, t4541: f64, t5962: f64, t5966: f64, t6075: f64, t6079: f64, t7432: f64, t8020: f64, t892: f64, t95964: f64) -> f64 {
    let t115819 = -3.0_f64 * t1940 * t110177 * t1583 + 18.0_f64 * t2403 * t26590 * t113440 + 18.0_f64 * t4541 * t2071 * t23279 + 9.0_f64 * t2403 * t8020 * t5962 + 6.0_f64 * t198 * t23114 * t2070 * t892 + 18.0_f64 * t4541 * t8020 * t5966 + 9.0_f64 * t2403 * t30420 * t1544 + t198 * t207 * t115747 * t892 - 18.0_f64 * t2403 * t28460 * t29598 - 3.0_f64 * t1940 * t28460 * t6075 - 9.0_f64 * t2403 * t7432 * t113432 - 9.0_f64 * t2403 * t7432 * t113103 - 6.0_f64 * t1940 * t95964 * t23429 + 6.0_f64 * t1940 * t26590 * t113107 + 3.0_f64 * t2403 * t2071 * t23148 + 6.0_f64 * t1940 * t103586 * t6079 - t1940 * t7432 * t23421 - 18.0_f64 * t4541 * t7432 * t113096;
    t115819
}
