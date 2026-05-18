//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 299/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk299<F: Float>(t132: F, t425: F, t391: F, t88: F, t69: F, t62: F, t402: F, t106: F, t19: F, t65: F, t20: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t1134 = t132 * t425;
    let t1141 = t88 * t391;
    let t1144 = t69 * t69;
    let t1145 = F::new(1.0) / t1144;
    let t1146 = t62 * t1145;
    let t1147 = t402 * t402;
    let t1150 = F::new(1.0) / t106;
    let t1152 = t1150 * t65 * t19;
    let t1153 = t20 * t5;
    (t1134, t1141, t1146, t1147, t1150, t1152, t1153)
}
