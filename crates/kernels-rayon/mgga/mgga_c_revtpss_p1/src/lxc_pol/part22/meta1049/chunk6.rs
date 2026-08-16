//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3692/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3692(t1263: f64, t372: f64, t6628: f64, t19680: f64, t5405: f64, t20823: f64, t21233: f64, t3647: f64, t17451: f64, t17605: f64, t1261: f64, t12784: f64, t12866: f64, t13046: f64, t17381: f64, t17656: f64, t17693: f64, t17799: f64, t21022: f64, t247: f64, t3618: f64, t56758: f64, t56785: f64, t56787: f64, t56790: f64, t56793: f64, t56997: f64, t57710: f64, t68395: f64) -> (f64, f64, f64, f64) {
    let t69839 = t372 * t1263 * t6628;
    let t69844 = t19680 * t5405;
    let t69848 = t20823 * t5405;
    let t69856 = t3647 * t21233;
    let t69866 = t17605 * t17451;
    let t69868 = 0.3811023832717309953e-3_f64 * t56758 - 0.17149607247227894789e-2_f64 * t56997 * t69839 * t13046 * t17656 - 0.57165357490759649296e-3_f64 * t17693 * t17799 * t69844 + 0.57165357490759649296e-3_f64 * t12866 * t17799 * t69848 - 0.45732285992607719436e-2_f64 * t57710 * t17381 - 0.57165357490759649296e-3_f64 * t12784 * t21022 + 0.31758531939310916276e-3_f64 * t69856 + 0.47637797908966374414e-3_f64 * t1261 * t247 * t3618 * t68395 + 0.10162730220579493208e-2_f64 * t56785 + 0.3811023832717309953e-3_f64 * t56787 + 0.2540682555144873302e-3_f64 * t56790 - 0.84689418504829110067e-3_f64 * t56793 + 0.20325460441158986416e-2_f64 * t69866;
    (t69839, t69844, t69848, t69868)
}
