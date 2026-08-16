//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3297/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297<F: Float>(t5741: F, t75251: F, t47348: F, t47351: F, t47352: F, t47381: F, t49290: F, t75174: F, t75176: F, t75179: F, t75190: F, t75205: F) -> F {
    let t86563 = t75251 * t5741;
    let t86567 = -F::cast_from(0.16463622957338778996e-1_f64) * t75174 + F::cast_from(0.7805952431506226415e-1_f64) * t75176 - F::cast_from(0.7805952431506226415e-1_f64) * t75179 + F::cast_from(0.19637199382202157274e-3_f64) * t47348 - t47351 + F::cast_from(0.26019841438354088051e-2_f64) * t47352 - F::cast_from(0.65854491829355115984e-1_f64) * t75190 - t49290 - F::cast_from(0.29272321618148349057e-1_f64) * t86563 - F::cast_from(0.11044544084478153697e-3_f64) * t47381 + F::cast_from(0.16463622957338778996e-1_f64) * t75205;
    t86567
}
