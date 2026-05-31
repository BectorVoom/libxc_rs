//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 305/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk305<F: Float>(t198: F, t207: F, t679: F, t704: F, t709: F, t718: F, t751: F, t754: F, t759: F, t764: F, t765: F, t775: F, t890: F, t892: F) -> F {
    let t895 = t198 * t207 * t890 * t892 + F::cast_from(3.0_f64) * t198 * t765 * t775 + t679 + t704 + t709 + t718 + t751 + t754 - t759 - t764;
    t895
}
