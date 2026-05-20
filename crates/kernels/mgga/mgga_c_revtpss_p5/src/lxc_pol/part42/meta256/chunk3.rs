//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 981/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981<F: Float>(t1518: F, t8342: F, t117: F, t8406: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t8421: F, t587: F, t65: F) -> (F, F, F, F) {
    let t8427 = t8342 * t1518;
    let t8430 = t117 * t8406;
    let t8433 = F::new(3.0) * t1916 * t2209 + F::new(3.0) * t1918 * t2207 + F::new(6.0) * t572 * t8427 + F::new(3.0) * t572 * t8430 + t573 * t8421;
    let t8779 = F::new(1.0) / t65 / t587;
    (t8427, t8430, t8433, t8779)
}
