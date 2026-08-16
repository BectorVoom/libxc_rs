//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3003/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3003<F: Float>(t1063: F, t15790: F, t3172: F, t11223: F, t16088: F, t380: F, t1041: F, t16185: F, t11202: F, t11637: F, t11774: F, t11933: F, t15139: F, t16078: F, t16091: F, t16095: F, t19980: F, t3092: F, t3117: F, t357: F, t42410: F, t42571: F, t43017: F, t43019: F, t43057: F, t43291: F, t4573: F, t4781: F, t4875: F) -> F {
    let t54849 = t1063 * t3172 * t15790;
    let t54857 = t11223 * t380 * t16088;
    let t54869 = t1041 * t3172 * t16185;
    let t54880 = -F::cast_from(0.11433071498151929859e-2_f64) * t54849 + F::cast_from(0.47637797908966374413e-3_f64) * t43017 + F::cast_from(0.96545937095505185477e-2_f64) * t43019 - F::cast_from(0.7145669686344956162e-3_f64) * t11774 * t19980 * t43057 + F::cast_from(0.17149607247227894789e-2_f64) * t54857 * t16091 - F::cast_from(0.25724410870841842183e-2_f64) * t16095 * t3092 * t4573 * t11637 + F::cast_from(0.34299214494455789577e-2_f64) * t11933 * t16078 + F::cast_from(0.45732285992607719436e-2_f64) * t42571 * t4875 + F::cast_from(0.42874018118069736972e-3_f64) * t54869 - F::cast_from(0.19055119163586549765e-2_f64) * t16095 * t42410 * t15139 * t11637 - F::cast_from(0.12862205435420921092e-2_f64) * t43291 * t3117 * t4781 * t357 * t11202;
    t54880
}
