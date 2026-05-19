//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 798/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk798<F: Float>(t114: F, t10254: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t10210: F, t10214: F, t69: F) -> F {
    let t115 = F::new(1.0) < t114;
    let t10255 = t655 * t10254;
    let t10259 = piecewise3::<F>(t115, F::new(0.0), -t10201 - F::new(11.0) / F::new(3.0) * t10202 - F::new(2.0) * t10204 + t10206 - F::new(3.0) / F::new(4.0) * t69 * t10210 + F::new(3.0) / F::new(4.0) * t69 * t10214 - t69 * t10255 / F::new(8.0));
    t10259
}
