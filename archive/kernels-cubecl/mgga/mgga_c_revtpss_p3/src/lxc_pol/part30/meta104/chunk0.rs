//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 641/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk641<F: Float>(t2398: F, t707: F, t150: F, t2389: F, t190: F, t198: F, t206: F) -> (F, F, F, F) {
    let t2400 = F::cast_from(8.0_f64) * t2398 * t707;
    let t2401 = t150 * t2389;
    let t2402 = t2401 * t190;
    let t2403 = t198 * t206;
    (t2400, t2401, t2402, t2403)
}
