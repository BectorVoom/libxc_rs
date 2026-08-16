//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3534/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534(t20020: f64, t3211: f64, t15656: f64, t4845: f64, t19675: f64, t372: f64, t11774: f64, t11779: f64, t11933: f64, t15703: f64, t15745: f64, t16067: f64, t16068: f64, t1665: f64, t20091: f64, t3096: f64, t3117: f64, t4854: f64, t54699: f64, t54907: f64, t54914: f64, t54919: f64, t54925: f64, t6278: f64, t65144: f64, t66542: f64) -> f64 {
    let t67044 = t3211 * t20020;
    let t67048 = t15656 * t4845;
    let t67052 = t372 * t19675;
    let t67058 = 0.21437009059034868486e-3_f64 * t16067 * t3117 * t65144 * t16068 - 0.19055119163586549765e-3_f64 * t54907 + 0.45732285992607719436e-2_f64 * t11933 * t20091 - 0.57165357490759649296e-3_f64 * t54914 - 0.57165357490759649296e-3_f64 * t54919 - 0.7622047665434619906e-3_f64 * t54925 - 0.72409452821628889107e-2_f64 * t11779 * t6278 + 0.15244095330869239812e-2_f64 * t67044 + 0.45732285992607719436e-2_f64 * t54699 * t1665 - 0.57165357490759649296e-3_f64 * t67048 + 0.45732285992607719436e-2_f64 * t15745 * t4854 - 0.28582678745379824648e-3_f64 * t11774 * t67052 * t3096 - 0.11433071498151929859e-2_f64 * t66542 * t15703;
    t67058
}
