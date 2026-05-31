//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 971/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk971<F: Float>(t1453: F, t2178: F, t1312: F, t2179: F, t2181: F, t2322: F, t4254: F, t5523: F, t651: F, t8254: F, t8274: F, t8278: F) -> (F, F) {
    let t8280 = t2178 * t1453;
    let t8283 = F::cast_from(2.0_f64) * t1312 * t8278 + F::cast_from(2.0_f64) * t1312 * t8280 - F::cast_from(2.0_f64) * t2179 * t2322 - F::cast_from(2.0_f64) * t2179 * t4254 + F::cast_from(2.0_f64) * t2181 * t2322 + F::cast_from(2.0_f64) * t2181 * t5523 - F::cast_from(2.0_f64) * t651 * t8254 - F::cast_from(2.0_f64) * t651 * t8274;
    (t8280, t8283)
}
