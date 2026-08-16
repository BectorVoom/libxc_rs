//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 873/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk873<F: Float>(t159: F, t3181: F, t2851: F, t631: F, t10356: F, t128: F) -> (F, F, F) {
    let t11142 = t159 * t3181;
    let t11144 = F::cast_from(1.0_f64) / t2851 / t631;
    let t11145 = t11144 * t10356;
    let t11146 = t11142 * t11145;
    let t11147 = t128 * t11146;
    (t11144, t11145, t11147)
}
