//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1014/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1014(t114792: f64, t114795: f64, t116536: f64, t121413: f64, t121419: f64, t121426: f64, t121429: f64, t121431: f64, t121435: f64, t121437: f64, t121444: f64, t121448: f64, t121457: f64, t121464: f64, t24297: f64, t32002: f64, t4268: f64, t7830: f64) -> f64 {
    let t123521 = 0.6579736267392905746e-1_f64 * t121413 - 0.13159472534785811492e0_f64 * t121419 + 4.0_f64 * t24297 * t7830 + 0.6579736267392905746e-1_f64 * t121426 + 0.6579736267392905746e-1_f64 * t121429 + 0.76763589786250567037e-1_f64 * t121431 + 0.6579736267392905746e-1_f64 * t121435 - 0.15352717957250113407e0_f64 * t121437 + 0.16449340668482264365e-1_f64 * t114792 + 0.16449340668482264365e-1_f64 * t114795 - 0.3289868133696452873e-1_f64 * t121444 + 0.6579736267392905746e-1_f64 * t121448 + 4.0_f64 * t4268 * t32002 - t116536 - 0.3289868133696452873e-1_f64 * t121457 + 0.3289868133696452873e-1_f64 * t121464;
    t123521
}
