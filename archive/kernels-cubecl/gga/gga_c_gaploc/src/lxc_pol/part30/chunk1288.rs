//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1288/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1288<F: Float>(t32893: F, t7427: F, t7573: F, t10892: F, t1980: F, t11026: F, t5782: F, t11030: F, t2365: F, t24741: F, t6111: F, t32514: F, t6066: F, t7630: F) -> (F, F, F, F, F, F) {
    let t33205 = F::cast_from(0.62115540045351614476e2_f64) * t7427 * t7573 * t32893;
    let t33206 = t1980 * t10892;
    let t33210 = F::cast_from(0.13803453343411469884e2_f64) * t5782 * t11026;
    let t33212 = F::cast_from(0.13803453343411469884e2_f64) * t5782 * t11030;
    let t33214 = t6111 * t2365 * t24741;
    let t33215 = F::cast_from(0.59584149919750711116e-1_f64) * t33214;
    let t33218 = F::cast_from(0.85801175884441024006e1_f64) * t7630 * t6066 * t32514;
    (t33205, t33206, t33210, t33212, t33215, t33218)
}
