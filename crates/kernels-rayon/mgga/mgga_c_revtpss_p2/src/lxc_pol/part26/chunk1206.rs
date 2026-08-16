//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1206/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1206(t26230: f64, t94764: f64, t94768: f64, t94763: f64, t26234: f64, t94890: f64, t25904: f64, t96245: f64, t94418: f64, t94420: f64, t94424: f64, t94426: f64, t94430: f64, t94432: f64, t94434: f64, t94436: f64, t94438: f64, t94440: f64, t94444: f64, t94446: f64, t94449: f64, t94451: f64) -> (f64, f64, f64, f64, f64) {
    let t96291 = t26230 * t94764;
    let t96292 = t94768 * t96291;
    let t96294 = t94763 * t96291;
    let t96296 = t94890 * t26234;
    let t96298 = t25904 * t96245;
    let t96314 = 0.10289764348336736873e-1_f64 * t94418 + 0.10289764348336736873e-1_f64 * t94420 + 0.12196800674228478774e-2_f64 * t94424 - 0.51448821741683684367e-1_f64 * t94426 - 0.96037800584476210818e-1_f64 * t94430 - 0.20579528696673473747e-1_f64 * t94432 + 0.51448821741683684367e-2_f64 * t94434 - 0.25724410870841842183e-2_f64 * t94436 + 0.10289764348336736873e-1_f64 * t94438 + 0.51448821741683684367e-2_f64 * t94440 + 0.65049603595885220128e-2_f64 * t94444 - 0.10289764348336736873e0_f64 * t94446 + 0.85748036236139473944e-4_f64 * t94449 - 0.34299214494455789578e-2_f64 * t94451;
    (t96292, t96294, t96296, t96298, t96314)
}
