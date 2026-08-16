//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3692/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3692<F: Float>(t1263: F, t372: F, t6628: F, t19680: F, t5405: F, t20823: F, t21233: F, t3647: F, t17451: F, t17605: F, t1261: F, t12784: F, t12866: F, t13046: F, t17381: F, t17656: F, t17693: F, t17799: F, t21022: F, t247: F, t3618: F, t56758: F, t56785: F, t56787: F, t56790: F, t56793: F, t56997: F, t57710: F, t68395: F) -> (F, F, F, F) {
    let t69839 = t372 * t1263 * t6628;
    let t69844 = t19680 * t5405;
    let t69848 = t20823 * t5405;
    let t69856 = t3647 * t21233;
    let t69866 = t17605 * t17451;
    let t69868 = F::cast_from(0.3811023832717309953e-3_f64) * t56758 - F::cast_from(0.17149607247227894789e-2_f64) * t56997 * t69839 * t13046 * t17656 - F::cast_from(0.57165357490759649296e-3_f64) * t17693 * t17799 * t69844 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t17799 * t69848 - F::cast_from(0.45732285992607719436e-2_f64) * t57710 * t17381 - F::cast_from(0.57165357490759649296e-3_f64) * t12784 * t21022 + F::cast_from(0.31758531939310916276e-3_f64) * t69856 + F::cast_from(0.47637797908966374414e-3_f64) * t1261 * t247 * t3618 * t68395 + F::cast_from(0.10162730220579493208e-2_f64) * t56785 + F::cast_from(0.3811023832717309953e-3_f64) * t56787 + F::cast_from(0.2540682555144873302e-3_f64) * t56790 - F::cast_from(0.84689418504829110067e-3_f64) * t56793 + F::cast_from(0.20325460441158986416e-2_f64) * t69866;
    (t69839, t69844, t69848, t69868)
}
