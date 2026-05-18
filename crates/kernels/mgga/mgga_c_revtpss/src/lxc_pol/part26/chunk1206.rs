//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1206/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1206<F: Float>(t26230: F, t94764: F, t94768: F, t94763: F, t26234: F, t94890: F, t25904: F, t96245: F, t94418: F, t94420: F, t94424: F, t94426: F, t94430: F, t94432: F, t94434: F, t94436: F, t94438: F, t94440: F, t94444: F, t94446: F, t94449: F, t94451: F) -> (F, F, F, F, F) {
    let t96291 = t26230 * t94764;
    let t96292 = t94768 * t96291;
    let t96294 = t94763 * t96291;
    let t96296 = t94890 * t26234;
    let t96298 = t25904 * t96245;
    let t96314 = F::new(0.10289764348336736873e-1) * t94418 + F::new(0.10289764348336736873e-1) * t94420 + F::new(0.12196800674228478774e-2) * t94424 - F::new(0.51448821741683684367e-1) * t94426 - F::new(0.96037800584476210818e-1) * t94430 - F::new(0.20579528696673473747e-1) * t94432 + F::new(0.51448821741683684367e-2) * t94434 - F::new(0.25724410870841842183e-2) * t94436 + F::new(0.10289764348336736873e-1) * t94438 + F::new(0.51448821741683684367e-2) * t94440 + F::new(0.65049603595885220128e-2) * t94444 - F::new(0.10289764348336736873e0) * t94446 + F::new(0.85748036236139473944e-4) * t94449 - F::new(0.34299214494455789578e-2) * t94451;
    (t96292, t96294, t96296, t96298, t96314)
}
