//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1436/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1436<F: Float>(t1263: F, t3362: F, t3172: F, t5298: F, t3711: F, t5278: F, t5269: F, t1261: F, t12256: F, t13099: F, t1224: F, t140: F) -> (F, F, F, F, F, F) {
    let t17202 = t1263 * t3362;
    let t17209 = t3172 * t5298;
    let t17211 = F::cast_from(0.19055119163586549765e-3_f64) * t3711 * t17209;
    let t17217 = t3172 * t5278;
    let t17219 = F::cast_from(0.19055119163586549765e-3_f64) * t3711 * t17217;
    let t17225 = t3172 * t5269;
    let t17227 = F::cast_from(0.3811023832717309953e-3_f64) * t1261 * t17225;
    let t17235 = t13099 * t12256;
    let t17240 = t140 * t1224;
    (t17202, t17211, t17219, t17227, t17235, t17240)
}
