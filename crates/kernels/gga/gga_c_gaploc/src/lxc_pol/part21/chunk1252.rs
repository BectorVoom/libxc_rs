//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1252/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1252<F: Float>(t11030: F, t5782: F, t2365: F, t24741: F, t6111: F, t32514: F, t6066: F, t7630: F, t10827: F, t825: F, t826: F, t3489: F, t6100: F) -> (F, F, F, F, F) {
    let t33212 = F::new(0.13803453343411469884e2) * t5782 * t11030;
    let t33214 = t6111 * t2365 * t24741;
    let t33215 = F::new(0.59584149919750711116e-1) * t33214;
    let t33218 = F::new(0.85801175884441024006e1) * t7630 * t6066 * t32514;
    let t33220 = t825 * t826 * t10827;
    let t33221 = F::new(0.51123901271894332902e0) * t33220;
    let t33222 = t6100 * t3489;
    (t33212, t33215, t33218, t33221, t33222)
}
