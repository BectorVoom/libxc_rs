//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1015/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1015<F: Float>(t1029: F, t9796: F, t9829: F, t3431: F, t5241: F, t2679: F, t9805: F, t20671: F, t28069: F, t33148: F, t10736: F, t28412: F, t913: F) -> (F, F, F, F) {
    let t43416 = t9796 * t1029 * t9829;
    let t43417 = F::cast_from(0.76685851907841499353e0_f64) * t43416;
    let t43419 = t5241 * t3431;
    let t43421 = t9805 * t43419 * t2679;
    let t43425 = t28069 * t20671 * t33148;
    let t43426 = F::cast_from(0.42603251059911944084e0_f64) * t43425;
    let t43432 = t28412 * t913 * t10736;
    (t43417, t43421, t43426, t43432)
}
