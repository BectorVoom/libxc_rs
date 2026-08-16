//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2233/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2233<F: Float>(t104695: F, t13142: F, t17384: F, t26867: F, t17640: F, t17646: F, t17690: F, t17705: F, t17750: F, t17781: F, t26852: F, t29097: F, t29100: F, t5304: F, t5354: F, t5402: F, t97182: F, t97187: F, t97232: F) -> F {
    let t104774 = t13142 * t104695;
    let t104793 = F::cast_from(0.3811023832717309953e-3_f64) * t26867 * t17384;
    let t104796 = -F::cast_from(0.25724410870841842183e-2_f64) * t104774 * t17750 - F::cast_from(0.85748036236139473944e-3_f64) * t97182 * t5354 - F::cast_from(0.57165357490759649296e-3_f64) * t97187 + F::cast_from(0.95275595817932748826e-3_f64) * t26852 * t5304 + F::cast_from(0.85748036236139473944e-3_f64) * t29097 * t17705 - F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t17640 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t17646 - F::cast_from(0.57165357490759649296e-3_f64) * t97232 * t5402 + F::cast_from(0.47637797908966374413e-3_f64) * t26867 * t17690 - t104793 - F::cast_from(0.85748036236139473944e-3_f64) * t29100 * t17781;
    t104796
}
