//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1332/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1332<F: Float>(t12254: F, t20293: F, t141: F, t12542: F, t12543: F, t16710: F, t16931: F, t17131: F, t17140: F, t20366: F, t20368: F, t20371: F, t20373: F) -> (F, F) {
    let t20377 = t12254 * t20293;
    let t20378 = t141 * t20377;
    let t20380 = -F::cast_from(0.412621875e-1_f64) * t20366 + F::new(0.16504875e0) * t20368 + F::new(0.82524375e-1) * t20371 - t17131 - t12542 - t12543 + F::new(0.16504875e0) * t20373 - F::cast_from(0.40256666666666666668e0_f64) * t16710 + t17140 + F::cast_from(0.36793333333333333333e-1_f64) * t16931 + F::cast_from(0.36793333333333333333e-1_f64) * t20378;
    (t20378, t20380)
}
