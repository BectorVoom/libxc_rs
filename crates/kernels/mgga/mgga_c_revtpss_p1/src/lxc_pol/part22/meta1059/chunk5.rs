//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3768/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768<F: Float>(t3588: F, t6587: F, t6573: F, t12916: F, t20801: F, t5340: F, t20805: F, t5331: F, t12784: F, t21090: F, t1250: F, t12787: F, t12866: F, t12910: F, t13392: F, t15936: F, t17534: F, t17694: F, t17729: F, t17742: F, t20921: F, t21035: F, t21040: F, t3626: F, t3718: F, t3720: F, t5046: F, t5330: F, t5343: F, t59358: F, t59360: F, t59492: F, t69848: F) -> (F, F, F) {
    let t71940 = t6587 * t3588;
    let t71945 = t6573 * t3588;
    let t71971 = t5340 * t12916 * t20801;
    let t71974 = t5331 * t12916 * t20805;
    let t71976 = t12784 * t21090;
    let t71981 = F::cast_from(0.30488190661738479624e-2_f64) * t59358 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t71940 * t1250 + F::cast_from(0.42874018118069736972e-3_f64) * t12910 * t3720 * t71945 * t1250 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t21040 * t17742 + F::cast_from(0.57165357490759649296e-3_f64) * t59360 + F::cast_from(0.17149607247227894789e-2_f64) * t59492 * t5330 * t5343 + F::cast_from(0.57165357490759649296e-3_f64) * t17729 * t3626 * t21035 * t13392 - F::cast_from(0.95275595817932748828e-3_f64) * t17729 * t12787 * t5046 * t17534 + F::cast_from(0.17149607247227894789e-2_f64) * t17729 * t3626 * t20921 * t15936 + F::cast_from(0.57165357490759649296e-3_f64) * t71971 - F::cast_from(0.28582678745379824648e-3_f64) * t71974 - F::cast_from(0.3811023832717309953e-3_f64) * t71976 - F::cast_from(0.47637797908966374414e-3_f64) * t12866 * t17694 * t69848;
    (t71940, t71945, t71981)
}
