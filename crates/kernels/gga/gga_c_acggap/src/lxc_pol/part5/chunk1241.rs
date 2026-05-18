//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1241/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1241<F: Float>(t2937: F, t513: F, t506: F, t3706: F, t495: F, t1165: F, t1173: F, t1181: F, t1532: F, t1552: F, t17388: F, t17390: F, t17392: F, t17395: F, t17397: F, t17399: F, t17404: F, t17409: F, t1894: F, t3196: F, t3396: F, t4450: F, t5012: F, t943: F) -> F {
    let t22721 = t2937 * t513;
    let t22731 = t2937 * t506;
    let t22737 = t3706 * t495;
    let t22750 = F::new(0.51448821741683684367e-2) * t4450 * t1165 * t1552 * t22721 * t943 + F::new(0.34299214494455789578e-2) * t1173 * t1181 * t1894 * t3196 - F::new(0.51448821741683684367e-2) * t4450 * t1181 * t1532 * t22731 * t943 + F::new(0.41159057393346947492e-1) * t3396 * t1165 * t22737 * t5012 + F::new(0.17149607247227894789e-2) * t17388 + F::new(0.17149607247227894789e-2) * t17390 - F::new(0.18140473443734395377e0) * t17392 + F::new(0.18140473443734395377e0) * t17395 - F::new(0.16006300097412701803e0) * t17397 - F::new(0.80031500487063509016e-1) * t17399 + F::new(0.34299214494455789578e-2) * t17404 + F::new(0.17149607247227894789e-2) * t17409;
    t22750
}
