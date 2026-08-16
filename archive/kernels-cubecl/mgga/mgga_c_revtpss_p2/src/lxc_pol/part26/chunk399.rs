//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 399/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk399<F: Float>(t1312: F, t2052: F, t2055: F, t2016: F, t2020: F) -> (F, F) {
    let t2093 = F::cast_from(2.0_f64) * t1312 * t2055 + t2052;
    let t2097 = t2016 / F::cast_from(48.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t2020;
    (t2093, t2097)
}
