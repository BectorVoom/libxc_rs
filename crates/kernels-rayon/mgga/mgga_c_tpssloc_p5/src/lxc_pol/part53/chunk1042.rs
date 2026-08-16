//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1042/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1042(t115658: f64, t117317: f64, t122206: f64, t122384: f64, t122390: f64, t122394: f64, t122399: f64, t122406: f64, t122551: f64, t122562: f64, t124223: f64, t124245: f64, t124273: f64, t1375: f64, t1378: f64, t16022: f64, t24095: f64, t26224: f64, t26990: f64, t3887: f64, t5325: f64, t5353: f64, t7213: f64, t7925: f64, t7936: f64, t8794: f64, t8800: f64) -> f64 {
    let t124281 = -0.6579736267392905746e-1_f64 * t122384 + 0.16449340668482264365e-1_f64 * t122390 + 2.0_f64 * t16022 * t8794 - 0.13159472534785811492e0_f64 * t122394 - 0.19739208802178717238e0_f64 * t122399 + 4.0_f64 * t24095 * t7925 + t117317 - 0.3289868133696452873e-1_f64 * t122406 + 2.0_f64 * t1375 * t3887 * t8800 * t5353 + 4.0_f64 * t1375 * t3887 * t7213 * t7936 - 6.0_f64 * t26224 * t124223 * t5325 - 0.16449340668482264365e-1_f64 * t122551 - t1375 * t1378 * (t124245 + t124273) - 0.16449340668482264365e-1_f64 * t115658 - 0.3289868133696452873e-1_f64 * t122562 - 12.0_f64 * t122206 * t26990;
    t124281
}
