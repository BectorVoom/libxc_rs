//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1077/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1077<F: Float>(t26291: F, t26374: F, t532: F, t1450: F, t1310: F, t18163: F, t2014: F, t2056: F, t2089: F, t2093: F, t2320: F, t2322: F, t2328: F, t2372: F, t26154: F, t26162: F, t26210: F, t26218: F, t26223: F, t4151: F, t4254: F, t508: F, t649: F, t651: F, t7235: F, t7357: F, t7359: F, t7367: F, t7374: F, t7378: F, t7474: F, t7489: F, t7539: F) -> (F, F, F, F) {
    let t26375 = t26291 + t26374;
    let t26376 = t532 * t26375;
    let t26377 = t26376 * t1450;
    let t26379 = -F::cast_from(2.0_f64) * t1310 * t7357 - F::cast_from(2.0_f64) * t18163 * t2056 + F::cast_from(6.0_f64) * t2014 * t26162 + t2014 * t26377 - t2089 * t2320 - F::cast_from(2.0_f64) * t2089 * t2328 + t2093 * t4151 - F::cast_from(4.0_f64) * t2322 * t7374 - F::cast_from(4.0_f64) * t2322 * t7378 - F::cast_from(2.0_f64) * t2372 * t7359 - F::cast_from(2.0_f64) * t26154 * t651 - t26210 * t508 - F::cast_from(2.0_f64) * t26218 * t651 - F::cast_from(4.0_f64) * t26223 * t651 - F::cast_from(4.0_f64) * t4254 * t7367 - F::cast_from(4.0_f64) * t4254 * t7374 - F::cast_from(2.0_f64) * t649 * t7474 + F::cast_from(6.0_f64) * t7235 * t7489 - F::cast_from(2.0_f64) * t7235 * t7539;
    (t26375, t26376, t26377, t26379)
}
