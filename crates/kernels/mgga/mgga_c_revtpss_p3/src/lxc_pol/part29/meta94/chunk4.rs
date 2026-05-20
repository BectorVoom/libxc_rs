//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 577/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk577<F: Float>(t265: F, t393: F, t2071: F, t30: F, t207: F, t2070: F, t198: F, t892: F) -> (F, F, F, F) {
    let t394 = t265 < t393;
    let t2072 = t2071 * t30;
    let t2075 = t207 * t2070;
    let t2077 = t198 * t2075 * t892;
    let t2078 = piecewise3::<F>(t394, F::new(0.0), t2077);
    (t2072, t2075, t2077, t2078)
}
