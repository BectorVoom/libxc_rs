//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3502/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502<F: Float>(t15669: F, t16088: F, t380: F, t1042: F, t1063: F, t11703: F, t11994: F, t15707: F, t16091: F, t16095: F, t16096: F, t16144: F, t18908: F, t19672: F, t19693: F, t3106: F, t4801: F, t4837: F, t51958: F, t53690: F, t65947: F, t66017: F, t66022: F, t66024: F, t66029: F, t66037: F, t66043: F) -> F {
    let t66047 = t15669 * t380 * t16088;
    let t66054 = F::cast_from(0.28582678745379824648e-3_f64) * t66017 - F::cast_from(0.47637797908966374414e-3_f64) * t11994 * t19693 - F::cast_from(0.47637797908966374413e-4_f64) * t66022 + F::cast_from(0.15244095330869239812e-2_f64) * t66024 - F::cast_from(0.67751534803863288055e-2_f64) * t3106 * t19672 - F::cast_from(0.95275595817932748827e-4_f64) * t66029 - F::cast_from(0.34299214494455789578e-2_f64) * t1063 * t1042 * t51958 * t65947 + F::cast_from(0.57165357490759649296e-3_f64) * t15707 * t16144 - F::cast_from(0.57165357490759649296e-3_f64) * t4837 * t1042 * t4801 * t66037 + F::cast_from(0.3811023832717309953e-3_f64) * t66043 + F::cast_from(0.3811023832717309953e-3_f64) * t53690 + F::cast_from(0.11433071498151929859e-2_f64) * t66047 * t16091 + F::cast_from(0.28582678745379824648e-2_f64) * t16095 * t11703 * t18908 * t16096;
    t66054
}
