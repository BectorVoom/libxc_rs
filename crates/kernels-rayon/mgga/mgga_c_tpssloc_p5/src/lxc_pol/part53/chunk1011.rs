//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1011/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1011(t225: f64, t33948: f64, t114592: f64, t114606: f64, t121296: f64, t121299: f64, t121302: f64, t121305: f64, t121308: f64, t121311: f64, t121314: f64, t121318: f64, t121326: f64, t13042: f64, t2054: f64, t866: f64, t8741: f64, t92847: f64, t92939: f64) -> f64 {
    let t123443 = t33948 * t225;
    let t123452 = -0.3289868133696452873e-1_f64 * t114592 + 0.76763589786250567037e-1_f64 * t121296 + 0.6579736267392905746e-1_f64 * t121299 - 0.3289868133696452873e-1_f64 * t121302 + 0.16449340668482264365e-1_f64 * t121305 - 0.3289868133696452873e-1_f64 * t121308 - 0.6579736267392905746e-1_f64 * t121311 - 0.6579736267392905746e-1_f64 * t121314 - 0.3289868133696452873e-1_f64 * t121318 - t123443 * t866 - 2.0_f64 * t92847 * t2054 - 0.13159472534785811492e0_f64 * t121326 - 0.15352717957250113407e0_f64 * t114606 - t13042 * t8741 - 2.0_f64 * t92939 * t2054;
    t123452
}
