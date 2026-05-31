//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2850/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2850<F: Float>(t61114: F, t18569: F, t4311: F, t22671: F, t706: F, t750: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t76944: F, t76946: F, t76948: F, t76950: F, t76951: F, t76952: F, t76954: F) -> (F, F, F, F) {
    let t76955 = F::cast_from(3.0_f64) * t61114;
    let t76957 = F::cast_from(12.0_f64) * t4311 * t18569;
    let t76959 = t706 * t750 * t22671;
    let t76960 = F::cast_from(4.0_f64) * t76959;
    let t76961 = -t76944 + t76946 + t76948 + t76950 + t76951 - t76952 - t39483 + t76954 + t39520 + t76955 - t39528 + t76957 + t39531 + t76960 + t39534 + t39537 - t39540;
    (t76955, t76957, t76960, t76961)
}
