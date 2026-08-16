//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1038/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1038(t124152: f64, t124165: f64, t115572: f64, t117287: f64, t117300: f64, t122251: f64, t122260: f64, t122270: f64, t122278: f64, t122281: f64, t122295: f64, t122304: f64, t124124: f64, t1375: f64, t1385: f64, t1386: f64, t24082: f64, t27062: f64, t33810: f64, t33843: f64, t3882: f64, t3887: f64, t539: f64, t568: f64, t7194: f64, t7937: f64) -> (f64, f64) {
    let t124166 = t124152 + t124165;
    let t124176 = 0.15352717957250113407e0_f64 * t122251 + t117287 - t124124 * t1386 - 2.0_f64 * t24082 * t7937 - 0.3289868133696452873e-1_f64 * t122260 - 6.0_f64 * t3882 * t33810 + 2.0_f64 * t1375 * t3887 * t33843 * t1385 + 0.6579736267392905746e-1_f64 * t122270 + t539 * t124166 * t568 + 0.6579736267392905746e-1_f64 * t122278 - 0.3289868133696452873e-1_f64 * t122281 + t117300 + 0.76763589786250567037e-1_f64 * t122295 + 0.16449340668482264365e-1_f64 * t115572 + 0.19739208802178717238e0_f64 * t122304 + 4.0_f64 * t7194 * t27062;
    (t124166, t124176)
}
