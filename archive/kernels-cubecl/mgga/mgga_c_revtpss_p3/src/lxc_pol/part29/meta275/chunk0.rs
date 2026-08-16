//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1133/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1133<F: Float>(t7076: F, t8011: F, t233: F, t7997: F, t1957: F, t1580: F, t1956: F, t2067: F, t213: F, t257: F, t7070: F, t7387: F, t7390: F, t7403: F, t7409: F, t7411: F, t7766: F, t7998: F, t8007: F) -> (F, F, F, F) {
    let t8012 = t7076 * t8011;
    let t8015 = t233 * t7997;
    let t8016 = t1957 * t8015;
    let t8019 = -t7387 + t7390 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t7998 * t257 - F::cast_from(0.65854491829355115987e0_f64) * t7403 * t1580 + t7409 - t7411 - F::cast_from(0.4336814094102599731e0_f64) * t7766 * t2067 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t8007 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t8012 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t8016;
    (t8012, t8015, t8016, t8019)
}
