//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1325/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1325<F: Float>(t31157: F, t569: F, t1453: F, t8320: F, t2198: F, t4151: F, t3813: F, t508: F, t1310: F, t10416: F, t1312: F, t13435: F, t13440: F, t18163: F, t2199: F, t2201: F, t2322: F, t4254: F, t5523: F, t651: F, t8307: F, t8321: F, t8325: F, t8327: F) -> (F, F, F, F, F, F, F) {
    let t31158 = t31157 * t569;
    let t31161 = t8320 * t1453;
    let t31164 = t2198 * t4151;
    let t31169 = t3813 * t2198;
    let t31172 = t508 * t31157;
    let t31201 = t1310 * t8320;
    let t31204 = -2.0 * t10416 * t2199 + 2.0 * t10416 * t2201 + 2.0 * t1312 * t31158 + 4.0 * t1312 * t31161 + 2.0 * t1312 * t31164 - 4.0 * t13435 * t2199 + 4.0 * t13435 * t2201 + 2.0 * t13440 * t2201 - 2.0 * t18163 * t2199 - 4.0 * t2322 * t8307 - 4.0 * t2322 * t8321 + 4.0 * t2322 * t8325 + 4.0 * t2322 * t8327 - 2.0 * t31169 * t651 - 2.0 * t31172 * t651 - 4.0 * t31201 * t651 - 4.0 * t4254 * t8307 - 4.0 * t4254 * t8321 + 4.0 * t5523 * t8325 + 4.0 * t5523 * t8327;
    (t31158, t31161, t31164, t31169, t31172, t31201, t31204)
}
