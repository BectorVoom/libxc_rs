//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1238/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1238<F: Float>(t2074: F, t810: F, t945: F, t2051: F, t2052: F, t18885: F, t18941: F, t18944: F, t18946: F, t18950: F, t18954: F, t18956: F, t18959: F, t19529: F, t19537: F, t19553: F, t19595: F, t19666: F, t19713: F, t19767: F, t19816: F, t19869: F, t19904: F, t19950: F, t19998: F, t20043: F, t20080: F, t20130: F, t21673: F, t21756: F, t21813: F, t21867: F, t2429: F, t321: F, t382: F, t4380: F, t6865: F, t6924: F, t804: F, t946: F) -> F {
    let t21875 = t945 * t810 * t2074;
    let t21883 = t2051 * t2051;
    let t21884 = t2052 * t2052;
    let t21885 = F::new(1.0) / t21884;
    let t21889 = -t19529 + t18941 - t19537 + t18944 + t18946 + F::new(3.0) * t804 * t382 * t19553 + t321 * (t19595 + t19666 + t19713 + t19767 + t19816 + t19869 + t19904 + t19950 + t19998 + t20043 + t20080 + t20130 + t21673 + t21756 + t21813 + t21867) * t945 - t18950 + t18954 + F::new(72.0) * t6924 * t21875 - t18956 + F::new(24.0) * t6865 * t946 + t18885 + F::new(24.0) * t2429 * t4380 * t810 - F::new(6.0) * t321 * t21883 * t21885 - t18959;
    t21889
}
