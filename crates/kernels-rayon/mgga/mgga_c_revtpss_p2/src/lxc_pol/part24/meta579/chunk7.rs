//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1792/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792(t12787: f64, t17693: f64, t17694: f64, t17729: f64, t17747: f64, t1785: f64, t20956: f64, t225: f64, t24647: f64, t24680: f64, t3720: f64, t480: f64, t484: f64, t5046: f64, t59144: f64, t71718: f64, t71744: f64, t84029: f64, t84032: f64, t84061: f64, t84645: f64, t89883: f64, t90881: f64) -> f64 {
    let t91378 = 0.28582678745379824648e-2_f64 * t17693 * t17694 * t90881 - 0.77173232612525526552e-2_f64 * t17747 * t3720 * t20956 * t84645 - 0.28582678745379824648e-2_f64 * t17729 * t12787 * t5046 * t24647 - 154.0_f64 / 243.0_f64 * t84029 - 10.0_f64 / 243.0_f64 * t59144 + 2.0_f64 / 27.0_f64 * t84032 - 2.0_f64 / 81.0_f64 * t71718 - 0.22866142996303859718e-2_f64 * t84061 - 0.96545937095505185475e-2_f64 * t71744 - 0.21240106161011140804e0_f64 * t1785 * t24680 * t484 + 0.21437009059034868486e-3_f64 * t89883 * t225 * t480 * t484;
    t91378
}
