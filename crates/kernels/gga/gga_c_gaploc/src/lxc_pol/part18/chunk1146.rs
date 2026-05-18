//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1146/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1146<F: Float>(t20957: F, t9294: F, t18823: F, t2476: F, t9438: F, t1457: F, t6655: F, t10525: F, t20370: F, t2365: F, t30110: F, t900: F, t9561: F) -> (F, F, F, F, F) {
    let t30835 = F::new(0.59584149919750711116e-1) * t20957 * t9294;
    let t30843 = t2476 * t9438 * t18823;
    let t30848 = t1457 * t6655;
    let t30897 = F::new(0.17875244975925213335e0) * t10525 * t2365 * t20370;
    let t30900 = F::new(0.20854452471912748891e0) * t9561 * t900 * t30110;
    (t30835, t30843, t30848, t30897, t30900)
}
