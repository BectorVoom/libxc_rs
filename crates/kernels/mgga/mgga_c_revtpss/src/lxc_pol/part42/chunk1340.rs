//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1340/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1340<F: Float>(t31398: F, t31461: F, t3: F, t2198: F, t670: F, t1518: F, t31234: F, t4292: F, t8342: F, t116: F, t8406: F, t117: F, t31451: F, t1459: F, t1461: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8336: F, t8343: F, t8346: F, t8421: F, t8427: F, t8430: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31463 = 2.0 * t31398 + 2.0 * t31461;
    let t31464 = t3 * t31463;
    let t31475 = param_d * t31463;
    let t31493 = t670 * t2198;
    let t31494 = t31493 * t1518;
    let t31497 = t31234 * t1518;
    let t31500 = t8342 * t4292;
    let t31505 = t116 * t8406;
    let t31506 = t31505 * t670;
    let t31509 = t117 * t31451;
    let t31512 = 6.0 * t1459 * t8427 + 3.0 * t1459 * t8430 + 3.0 * t1461 * t8421 + 6.0 * t1916 * t8343 + 3.0 * t1916 * t8346 + 3.0 * t1918 * t8336 + 6.0 * t2207 * t5802 + 3.0 * t2207 * t5805 + 3.0 * t2209 * t5795 + t31475 * t573 + 6.0 * t31494 * t572 + 6.0 * t31497 * t572 + 6.0 * t31500 * t572 + 6.0 * t31506 * t572 + 3.0 * t31509 * t572;
    (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509, t31512)
}
