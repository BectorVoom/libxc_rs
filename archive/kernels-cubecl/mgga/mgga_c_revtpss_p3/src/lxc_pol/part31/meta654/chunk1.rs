//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2187/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2187<F: Float>(t108076: F, t108078: F, t108080: F, t108083: F, t108085: F, t108087: F, t108089: F, t108099: F, t108103: F, t108105: F, t108107: F, t108109: F, t108111: F, t18235: F, t18242: F, t25805: F, t27145: F, t28025: F, t28053: F, t4248: F, t5921: F, t6985: F) -> F {
    let t108114 = -F::cast_from(4.0_f64) * t18235 * t6985 - F::cast_from(2.0_f64) * t18242 * t6985 - F::cast_from(2.0_f64) * t25805 * t5921 - F::cast_from(4.0_f64) * t27145 * t4248 - F::cast_from(2.0_f64) * t28025 * t5921 - F::cast_from(4.0_f64) * t28053 * t4248 - t108076 - t108078 - t108080 - t108083 - t108085 - t108087 - t108089 - t108099 + t108103 - t108105 - t108107 - t108109 - t108111;
    t108114
}
