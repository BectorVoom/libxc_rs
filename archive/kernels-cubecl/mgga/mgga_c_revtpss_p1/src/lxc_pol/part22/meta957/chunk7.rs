//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3214/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214<F: Float>(t10298: F, t10301: F, t10309: F, t13283: F, t13420: F, t1497: F, t21674: F, t21677: F, t21682: F, t2247: F, t2248: F, t2315: F, t4178: F, t4241: F, t45926: F, t45931: F, t45933: F, t45936: F, t45941: F, t45944: F, t45947: F, t45952: F, t45958: F, t45963: F, t45972: F, t5816: F, t5872: F, t60224: F, t603: F, t60793: F, t60829: F, t60871: F, t60994: F, t91: F) -> F {
    let t61007 = F::cast_from(40.0_f64) * t2247 * t1497 * t13420 + F::cast_from(40.0_f64) * t10301 * t21682 + F::cast_from(20.0_f64) * t45958 * t5816 - F::cast_from(480.0_f64) * t10309 * t4178 * t4241 - F::cast_from(4.0_f64) * t10298 * t5872 + F::cast_from(20.0_f64) * t2247 * t5872 * t2315 - F::cast_from(240.0_f64) * t60224 * t13283 + F::cast_from(840.0_f64) * t45972 * t5816 * t2248 - F::cast_from(240.0_f64) * t45963 * t21674 + F::cast_from(80.0_f64) * t10301 * t21677 - F::cast_from(4.0_f64) * t603 * (t60793 + t60829 + t60871 + t60994) - F::cast_from(120.0_f64) * t10309 * t5816 * t2315 + (-F::cast_from(24.0_f64) * t45926 + t45931 + t45933 - F::cast_from(480.0_f64) * t45936 + t45941 + t45944 - F::cast_from(2520.0_f64) * t45947 + t45952) * t91;
    t61007
}
