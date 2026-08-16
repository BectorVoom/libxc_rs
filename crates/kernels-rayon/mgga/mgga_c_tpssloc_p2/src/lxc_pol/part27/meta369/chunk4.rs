//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1524/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1524(t13644: f64, t13602: f64, t13598: f64, t13613: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t13647: f64, t10300: f64, t10542: f64, t10545: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13552: f64, t13557: f64, t13561: f64, t13616: f64, t13624: f64, t13626: f64, t13675: f64, t13679: f64, t13692: f64) -> f64 {
    let t13709 = 0.11038e0_f64 * t13644;
    let t13712 = 0.20128333333333333334e0_f64 * t13602;
    let t13714 = -0.258925e1_f64 * t13630 - 0.1294625e1_f64 * t13632 + 0.19419375e1_f64 * t13635 - 0.412621875e-1_f64 * t13638 + 0.258925e1_f64 * t13640 - 0.91983333333333333334e-1_f64 * t13642 + t13709 - 0.82785e-1_f64 * t13647 - 0.13418888888888888889e0_f64 * t13598 + t13712 - 0.301925e0_f64 * t13613;
    let t13716 = -0.5519e-1_f64 * t13530 - 0.27595e-1_f64 * t13534 - 0.36793333333333333333e-1_f64 * t13539 + 0.33114e0_f64 * t13544 + 0.16557e0_f64 * t13548 - t13675 + 0.36793333333333333334e-1_f64 * t13552 + 0.16557e0_f64 * t13557 - 0.49671e0_f64 * t13561 + t13679 + t13692 - t10542 - t10545 + 0.16504875e0_f64 * t13616 - 0.11038e0_f64 * t10300 - 0.26837777777777777778e0_f64 * t10556 + 0.67094444444444444447e-1_f64 * t10558 - 0.20128333333333333334e0_f64 * t10560 + 0.10064166666666666667e0_f64 * t10562 + 0.16504875e0_f64 * t13624 + 0.82524375e-1_f64 * t13626 + t13714;
    t13716
}
