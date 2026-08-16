//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 973/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk973<F: Float>(t1312: F, t2199: F, t2201: F, t2322: F, t4254: F, t5523: F, t651: F, t8307: F, t8321: F, t8325: F, t8327: F, t3: F) -> (F, F) {
    let t8330 = F::cast_from(2.0_f64) * t1312 * t8325 + F::cast_from(2.0_f64) * t1312 * t8327 - F::cast_from(2.0_f64) * t2199 * t2322 - F::cast_from(2.0_f64) * t2199 * t4254 + F::cast_from(2.0_f64) * t2201 * t2322 + F::cast_from(2.0_f64) * t2201 * t5523 - F::cast_from(2.0_f64) * t651 * t8307 - F::cast_from(2.0_f64) * t651 * t8321;
    let t8331 = t3 * t8330;
    (t8330, t8331)
}
