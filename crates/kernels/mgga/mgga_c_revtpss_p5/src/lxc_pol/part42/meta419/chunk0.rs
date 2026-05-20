//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1478/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1478<F: Float>(t31398: F, t31461: F, t3: F, t2198: F, t670: F, t1518: F, t31234: F, t4292: F, t8342: F, t116: F, t8406: F, t117: F, t31451: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31463 = F::new(2.0) * t31398 + F::new(2.0) * t31461;
    let t31464 = t3 * t31463;
    let t31475 = param_d * t31463;
    let t31493 = t670 * t2198;
    let t31494 = t31493 * t1518;
    let t31497 = t31234 * t1518;
    let t31500 = t8342 * t4292;
    let t31505 = t116 * t8406;
    let t31506 = t31505 * t670;
    let t31509 = t117 * t31451;
    (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509)
}
