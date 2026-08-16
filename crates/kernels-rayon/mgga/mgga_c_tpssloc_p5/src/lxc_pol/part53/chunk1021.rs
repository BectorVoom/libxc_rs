//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1021/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1021(t114916: f64, t114939: f64, t116645: f64, t116648: f64, t121689: f64, t121713: f64, t121716: f64, t123641: f64, t123663: f64, t13053: f64, t13065: f64, t1492: f64, t1528: f64, t2054: f64, t259: f64, t26582: f64, t26680: f64, t2718: f64, t31984: f64, t33973: f64, t4142: f64, t7087: f64, t855: f64, t858: f64, t865: f64, t8728: f64, t8734: f64, t92386: f64) -> f64 {
    let t123687 = 2.0_f64 * t855 * t2718 * t33973 * t865 - t855 * t858 * (t123641 + t123663) + t4142 * t8728 * t259 + t1492 * t31984 * t259 + 0.6579736267392905746e-1_f64 * t121689 + 0.3289868133696452873e-1_f64 * t114916 - t116645 * t1528 + 2.0_f64 * t13065 * t8734 - 2.0_f64 * t92386 * t2054 - t116648 + 0.76763589786250567037e-1_f64 * t114939 - 2.0_f64 * t7087 * t26680 - 0.3289868133696452873e-1_f64 * t121713 - 0.3289868133696452873e-1_f64 * t121716 + 4.0_f64 * t7087 * t26582 + 2.0_f64 * t13053 * t8734;
    t123687
}
