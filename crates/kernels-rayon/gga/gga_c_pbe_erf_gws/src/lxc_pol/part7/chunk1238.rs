//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1238/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1238(t2074: f64, t810: f64, t945: f64, t2051: f64, t2052: f64, t18885: f64, t18941: f64, t18944: f64, t18946: f64, t18950: f64, t18954: f64, t18956: f64, t18959: f64, t19529: f64, t19537: f64, t19553: f64, t19595: f64, t19666: f64, t19713: f64, t19767: f64, t19816: f64, t19869: f64, t19904: f64, t19950: f64, t19998: f64, t20043: f64, t20080: f64, t20130: f64, t21673: f64, t21756: f64, t21813: f64, t21867: f64, t2429: f64, t321: f64, t382: f64, t4380: f64, t6865: f64, t6924: f64, t804: f64, t946: f64) -> f64 {
    let t21875 = t945 * t810 * t2074;
    let t21883 = t2051 * t2051;
    let t21884 = t2052 * t2052;
    let t21885 = 1.0_f64 / t21884;
    let t21889 = -t19529 + t18941 - t19537 + t18944 + t18946 + 3.0_f64 * t804 * t382 * t19553 + t321 * (t19595 + t19666 + t19713 + t19767 + t19816 + t19869 + t19904 + t19950 + t19998 + t20043 + t20080 + t20130 + t21673 + t21756 + t21813 + t21867) * t945 - t18950 + t18954 + 72.0_f64 * t6924 * t21875 - t18956 + 24.0_f64 * t6865 * t946 + t18885 + 24.0_f64 * t2429 * t4380 * t810 - 6.0_f64 * t321 * t21883 * t21885 - t18959;
    t21889
}
