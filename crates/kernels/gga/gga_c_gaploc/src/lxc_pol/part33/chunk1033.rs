//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1033/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1033<F: Float>(t1457: F, t6655: F, t10525: F, t20370: F, t2365: F, t30110: F, t900: F, t9561: F, t20954: F, t3196: F, t1407: F, t9445: F, t1328: F, t20550: F, t6914: F, t9438: F) -> (F, F, F, F, F, F) {
    let t30848 = t1457 * t6655;
    let t30897 = 0.17875244975925213335e0 * t10525 * t2365 * t20370;
    let t30900 = 0.20854452471912748891e0 * t9561 * t900 * t30110;
    let t30901 = t20954 * t3196;
    let t30903 = t1407 * t9445;
    let t30907 = t6914 * t9438 * t20550 * t1328;
    (t30848, t30897, t30900, t30901, t30903, t30907)
}
