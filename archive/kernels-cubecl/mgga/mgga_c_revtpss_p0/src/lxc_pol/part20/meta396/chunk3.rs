//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1460/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460<F: Float>(t41316: F, t41323: F, t41353: F, t41356: F, t41359: F, t41396: F, t41402: F, t41404: F, t41406: F, t41409: F, t41412: F, t41414: F, t41417: F, t41419: F) -> F {
    let t41637 = -F::cast_from(0.107628e2_f64) * t41316 + F::cast_from(0.71752000000000000001e1_f64) * t41323 - F::cast_from(0.19931111111111111111e1_f64) * t41353 + F::cast_from(0.23917333333333333333e1_f64) * t41356 - F::cast_from(0.79724444444444444444e0_f64) * t41359 + F::cast_from(0.1898925e1_f64) * t41396 - F::cast_from(0.3560484375e1_f64) * t41402 - F::cast_from(0.28483875e1_f64) * t41404 + F::cast_from(0.21908444444444444444e0_f64) * t41406 - F::cast_from(0.295764e1_f64) * t41409 + F::cast_from(0.85451625e1_f64) * t41412 - F::cast_from(0.379785e1_f64) * t41414 - F::cast_from(0.46074375e0_f64) * t41417 + F::cast_from(0.614325e0_f64) * t41419;
    t41637
}
