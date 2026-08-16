//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2243/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2243<F: Float>(t21233: F, t7624: F, t29083: F, t5378: F, t21090: F, t26867: F, t104703: F, t104708: F, t104715: F, t104774: F, t17183: F, t20959: F, t20963: F, t21030: F, t21246: F, t29040: F, t29096: F, t5335: F, t5348: F, t5354: F, t5397: F, t97141: F, t97261: F) -> F {
    let t112232 = t7624 * t21233;
    let t112234 = t29083 * t5378;
    let t112243 = t26867 * t21090;
    let t112249 = -F::cast_from(0.85748036236139473944e-3_f64) * t104703 * t5348 + F::cast_from(0.25724410870841842183e-2_f64) * t104715 * t20959 - F::cast_from(0.25724410870841842183e-2_f64) * t104774 * t20963 + F::cast_from(0.63517063878621832551e-4_f64) * t97141 + F::cast_from(0.31758531939310916275e-3_f64) * t112232 + F::cast_from(0.20325460441158986416e-2_f64) * t112234 - F::cast_from(0.85748036236139473944e-3_f64) * t17183 * t29096 * t5335 + F::cast_from(0.45732285992607719436e-2_f64) * t104708 * t5354 + F::cast_from(0.85748036236139473944e-3_f64) * t29040 * t21246 - F::cast_from(0.38110238327173099531e-3_f64) * t112243 + F::cast_from(0.30488190661738479624e-2_f64) * t29083 * t5397 + F::cast_from(0.85748036236139473944e-3_f64) * t97261 * t21030;
    t112249
}
