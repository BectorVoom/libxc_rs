//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3214/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214<F: Float>(t10298: F, t10301: F, t10309: F, t13283: F, t13420: F, t1497: F, t21674: F, t21677: F, t21682: F, t2247: F, t2248: F, t2315: F, t4178: F, t4241: F, t45926: F, t45931: F, t45933: F, t45936: F, t45941: F, t45944: F, t45947: F, t45952: F, t45958: F, t45963: F, t45972: F, t5816: F, t5872: F, t60224: F, t603: F, t60793: F, t60829: F, t60871: F, t60994: F, t91: F) -> F {
    let t61007 = F::new(40.0) * t2247 * t1497 * t13420 + F::new(40.0) * t10301 * t21682 + F::new(20.0) * t45958 * t5816 - F::new(480.0) * t10309 * t4178 * t4241 - F::new(4.0) * t10298 * t5872 + F::new(20.0) * t2247 * t5872 * t2315 - F::new(240.0) * t60224 * t13283 + F::new(840.0) * t45972 * t5816 * t2248 - F::new(240.0) * t45963 * t21674 + F::new(80.0) * t10301 * t21677 - F::new(4.0) * t603 * (t60793 + t60829 + t60871 + t60994) - F::new(120.0) * t10309 * t5816 * t2315 + (-F::new(24.0) * t45926 + t45931 + t45933 - F::new(480.0) * t45936 + t45941 + t45944 - F::new(2520.0) * t45947 + t45952) * t91;
    t61007
}
