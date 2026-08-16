//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3768/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768(t3588: f64, t6587: f64, t6573: f64, t12916: f64, t20801: f64, t5340: f64, t20805: f64, t5331: f64, t12784: f64, t21090: f64, t1250: f64, t12787: f64, t12866: f64, t12910: f64, t13392: f64, t15936: f64, t17534: f64, t17694: f64, t17729: f64, t17742: f64, t20921: f64, t21035: f64, t21040: f64, t3626: f64, t3718: f64, t3720: f64, t5046: f64, t5330: f64, t5343: f64, t59358: f64, t59360: f64, t59492: f64, t69848: f64) -> (f64, f64, f64) {
    let t71940 = t6587 * t3588;
    let t71945 = t6573 * t3588;
    let t71971 = t5340 * t12916 * t20801;
    let t71974 = t5331 * t12916 * t20805;
    let t71976 = t12784 * t21090;
    let t71981 = 0.30488190661738479624e-2_f64 * t59358 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t71940 * t1250 + 0.42874018118069736972e-3_f64 * t12910 * t3720 * t71945 * t1250 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t21040 * t17742 + 0.57165357490759649296e-3_f64 * t59360 + 0.17149607247227894789e-2_f64 * t59492 * t5330 * t5343 + 0.57165357490759649296e-3_f64 * t17729 * t3626 * t21035 * t13392 - 0.95275595817932748828e-3_f64 * t17729 * t12787 * t5046 * t17534 + 0.17149607247227894789e-2_f64 * t17729 * t3626 * t20921 * t15936 + 0.57165357490759649296e-3_f64 * t71971 - 0.28582678745379824648e-3_f64 * t71974 - 0.3811023832717309953e-3_f64 * t71976 - 0.47637797908966374414e-3_f64 * t12866 * t17694 * t69848;
    (t71940, t71945, t71981)
}
