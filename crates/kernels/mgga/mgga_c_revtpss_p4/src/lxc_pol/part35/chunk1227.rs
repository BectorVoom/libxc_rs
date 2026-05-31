//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1227/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1227<F: Float>(t103586: F, t110177: F, t113096: F, t113103: F, t113107: F, t113432: F, t113440: F, t115747: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2070: F, t2071: F, t23114: F, t23148: F, t23279: F, t23421: F, t23429: F, t2403: F, t26590: F, t28460: F, t29598: F, t30420: F, t4541: F, t5962: F, t5966: F, t6075: F, t6079: F, t7432: F, t8020: F, t892: F, t95964: F) -> F {
    let t115819 = -F::cast_from(3.0_f64) * t1940 * t110177 * t1583 + F::cast_from(18.0_f64) * t2403 * t26590 * t113440 + F::cast_from(18.0_f64) * t4541 * t2071 * t23279 + F::cast_from(9.0_f64) * t2403 * t8020 * t5962 + F::cast_from(6.0_f64) * t198 * t23114 * t2070 * t892 + F::cast_from(18.0_f64) * t4541 * t8020 * t5966 + F::cast_from(9.0_f64) * t2403 * t30420 * t1544 + t198 * t207 * t115747 * t892 - F::cast_from(18.0_f64) * t2403 * t28460 * t29598 - F::cast_from(3.0_f64) * t1940 * t28460 * t6075 - F::cast_from(9.0_f64) * t2403 * t7432 * t113432 - F::cast_from(9.0_f64) * t2403 * t7432 * t113103 - F::cast_from(6.0_f64) * t1940 * t95964 * t23429 + F::cast_from(6.0_f64) * t1940 * t26590 * t113107 + F::cast_from(3.0_f64) * t2403 * t2071 * t23148 + F::cast_from(6.0_f64) * t1940 * t103586 * t6079 - t1940 * t7432 * t23421 - F::cast_from(18.0_f64) * t4541 * t7432 * t113096;
    t115819
}
