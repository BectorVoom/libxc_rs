//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 439/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk439<F: Float>(t1361: F, t1366: F, t1421: F, t1424: F, t1445: F, t213: F) -> F {
    let t1448 = -t1361 + t1366 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t1421 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t1445;
    t1448
}
