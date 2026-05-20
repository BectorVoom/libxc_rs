//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1504/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1504<F: Float>(t2198: F, t2371: F, t670: F, t8320: F, t117: F, t118019: F, t13514: F, t1459: F, t1518: F, t18204: F, t18208: F, t18211: F, t18214: F, t1916: F, t2207: F, t31231: F, t31235: F, t31238: F, t31493: F, t31494: F, t31506: F, t31509: F, t4158: F, t4162: F, t4292: F, t572: F, t5805: F, t8336: F, t8342: F, t8421: F, t8427: F) -> F {
    let t118157 = t2371 * t2198;
    let t118161 = t670 * t8320;
    let t118198 = F::new(3.0) * t117 * t118019 * t572 + F::new(6.0) * t118157 * t1518 * t572 + F::new(12.0) * t118161 * t1518 * t572 + F::new(6.0) * t13514 * t572 * t8342 + F::new(12.0) * t31493 * t4292 * t572 + F::new(12.0) * t1459 * t31494 + F::new(12.0) * t1459 * t31506 + F::new(6.0) * t1459 * t31509 + F::new(6.0) * t18204 * t2207 + F::new(12.0) * t18208 * t2207 + F::new(6.0) * t18211 * t2207 + F::new(3.0) * t18214 * t2207 + F::new(6.0) * t1916 * t31231 + F::new(12.0) * t1916 * t31235 + F::new(6.0) * t1916 * t31238 + F::new(6.0) * t4158 * t8427 + F::new(6.0) * t4162 * t8421 + F::new(6.0) * t5805 * t8336;
    t118198
}
