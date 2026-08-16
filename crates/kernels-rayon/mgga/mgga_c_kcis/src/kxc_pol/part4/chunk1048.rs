//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1048/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1048(t1014: f64, t4789: f64, t2820: f64, t4552: f64, t86: f64, t4557: f64, t4792: f64, t9415: f64, t3200: f64, t1133: f64, t167: f64, t3211: f64) -> (f64, f64, f64, f64, f64) {
    let t13238 = t1014 * t4789;
    let t13241 = t86 * t2820 * t4552;
    let t13242 = t13241 * t4557;
    let t13243 = 0.3684876543209876543e-2_f64 * t13242;
    let t13246 = t9415 * t4792;
    let t13247 = t3200 * t13246;
    let t13249 = t167 * t1133;
    let t13250 = t3211 * t13249;
    (t13238, t13242, t13243, t13247, t13250)
}
