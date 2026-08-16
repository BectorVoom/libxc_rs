//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 925/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk925(t21643: f64, t3201: f64, t3188: f64, t5866: f64, t1629: f64, t1058: f64, t11046: f64, t11059: f64, t11065: f64, t14608: f64, t14618: f64, t1610: f64, t1630: f64, t1632: f64, t18086: f64, t21481: f64, t21615: f64, t21618: f64, t21623: f64, t21627: f64, t21635: f64, t21638: f64, t21644: f64, t21647: f64, t21650: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4669: f64, t5903: f64, t5929: f64, t5933: f64, t5937: f64, t5939: f64, t5941: f64) -> f64 {
    let t21653 = t21643 * t3201;
    let t21656 = t3188 * t5866;
    let t21657 = t1629 * t21656;
    let t21662 = 3.0_f64 * t18086 * t1630 + 6.0_f64 * t14618 * t5929 - 3.0_f64 * t14608 * t5939 + t353 * t21615 + 3.0_f64 * t1058 * t21618 - 3.0_f64 * t3200 * t21623 + 3.0_f64 * t1058 * t21627 + 3.0_f64 * t1610 * t5941 + 3.0_f64 * t5903 * t1632 + t1058 * t21635 + t11046 * t21638 + t21481 * t384 + 6.0_f64 * t4669 * t5933 + 6.0_f64 * t3186 * t21644 + 6.0_f64 * t11059 * t21647 - 6.0_f64 * t11065 * t21650 - 3.0_f64 * t3200 * t21653 + 6.0_f64 * t3186 * t21657 + 3.0_f64 * t4669 * t5937;
    t21662
}
