//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 907/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk907<F: Float>(t20671: F, t28069: F, t33148: F, t10736: F, t28412: F, t913: F, t3038: F, t6119: F, t787: F, t9755: F, t41008: F, t2365: F, t33087: F, t8775: F) -> (F, F, F, F, F) {
    let t43425 = t28069 * t20671 * t33148;
    let t43426 = F::cast_from(0.42603251059911944084e0_f64) * t43425;
    let t43432 = t28412 * t913 * t10736;
    let t43433 = F::cast_from(0.59584149919750711116e-1_f64) * t43432;
    let t43440 = F::cast_from(0.27805936629216998521e0_f64) * t787 * t9755 * t3038 * t6119;
    let t43444 = F::cast_from(0.10352590007558602413e2_f64) * t41008;
    let t43446 = t8775 * t2365 * t33087;
    (t43426, t43433, t43440, t43444, t43446)
}
