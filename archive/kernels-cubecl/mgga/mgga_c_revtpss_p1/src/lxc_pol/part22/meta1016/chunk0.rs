//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3508/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508<F: Float>(t11675: F, t19785: F, t1043: F, t1045: F, t15145: F, t15691: F, t15700: F, t15895: F, t15957: F, t16017: F, t16226: F, t19501: F, t19741: F, t19776: F, t19934: F, t19998: F, t3091: F, t3092: F, t3155: F, t3188: F, t42580: F, t43175: F, t4583: F, t4892: F, t53800: F, t53993: F, t53998: F, t54026: F, t55100: F, t6266: F) -> F {
    let t66261 = t11675 * t19785;
    let t66263 = -F::cast_from(0.47637797908966374413e-4_f64) * t42580 + F::cast_from(0.3811023832717309953e-3_f64) * t53993 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t54026 * t6266 + F::cast_from(0.57165357490759649296e-3_f64) * t3091 * t3092 * t15957 * t19776 + F::cast_from(0.11433071498151929859e-2_f64) * t53998 - F::cast_from(0.11433071498151929859e-2_f64) * t3188 * t19934 - F::cast_from(0.85748036236139473944e-3_f64) * t19741 * t16017 - F::cast_from(0.85748036236139473944e-3_f64) * t53800 * t15895 - F::cast_from(0.11433071498151929859e-2_f64) * t15700 * t15691 * t1045 * t15145 + F::cast_from(0.11433071498151929859e-2_f64) * t16226 * t15691 * t3155 * t4583 * t1043 - F::cast_from(0.60976381323476959249e-2_f64) * t55100 * t19998 - F::cast_from(0.57165357490759649296e-3_f64) * t4892 * t3092 * t19501 * t43175 + F::cast_from(0.3811023832717309953e-3_f64) * t66261;
    t66263
}
