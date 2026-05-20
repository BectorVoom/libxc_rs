//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1775/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1775<F: Float>(t25282: F, t2736: F, t25251: F, t25254: F, t25257: F, t25258: F, t25263: F, t25267: F, t25271: F, t25276: F, t25279: F, t25280: F) -> (F, F) {
    let t25283 = t2736 * t25282;
    let t25284 = F::cast_from(0.50820002809285328225e-5_f64) * t25283;
    let t25285 = -F::cast_from(0.42874018118069736972e-3_f64) * t25251 + t25254 + t25257 - F::cast_from(0.42874018118069736972e-3_f64) * t25258 + F::cast_from(0.85748036236139473944e-3_f64) * t25263 + F::cast_from(0.40015750243531754508e-2_f64) * t25267 + F::cast_from(0.34299214494455789578e-2_f64) * t25271 + t25276 + t25279 - t25280 / F::new(48.0) - t25284;
    (t25284, t25285)
}
