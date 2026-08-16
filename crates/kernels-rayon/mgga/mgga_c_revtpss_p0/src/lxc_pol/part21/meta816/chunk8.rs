//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3003/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3003(t1063: f64, t15790: f64, t3172: f64, t11223: f64, t16088: f64, t380: f64, t1041: f64, t16185: f64, t11202: f64, t11637: f64, t11774: f64, t11933: f64, t15139: f64, t16078: f64, t16091: f64, t16095: f64, t19980: f64, t3092: f64, t3117: f64, t357: f64, t42410: f64, t42571: f64, t43017: f64, t43019: f64, t43057: f64, t43291: f64, t4573: f64, t4781: f64, t4875: f64) -> f64 {
    let t54849 = t1063 * t3172 * t15790;
    let t54857 = t11223 * t380 * t16088;
    let t54869 = t1041 * t3172 * t16185;
    let t54880 = -0.11433071498151929859e-2_f64 * t54849 + 0.47637797908966374413e-3_f64 * t43017 + 0.96545937095505185477e-2_f64 * t43019 - 0.7145669686344956162e-3_f64 * t11774 * t19980 * t43057 + 0.17149607247227894789e-2_f64 * t54857 * t16091 - 0.25724410870841842183e-2_f64 * t16095 * t3092 * t4573 * t11637 + 0.34299214494455789577e-2_f64 * t11933 * t16078 + 0.45732285992607719436e-2_f64 * t42571 * t4875 + 0.42874018118069736972e-3_f64 * t54869 - 0.19055119163586549765e-2_f64 * t16095 * t42410 * t15139 * t11637 - 0.12862205435420921092e-2_f64 * t43291 * t3117 * t4781 * t357 * t11202;
    t54880
}
