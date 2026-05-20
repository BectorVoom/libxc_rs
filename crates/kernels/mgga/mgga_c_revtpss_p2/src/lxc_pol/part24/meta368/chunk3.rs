//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1253/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1253<F: Float>(t12542: F, t12543: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F) -> F {
    let t24406 = F::new(0.181155e1) * t24242 + F::new(0.301925e0) * t24250 - F::new(0.16557e0) * t24289 + F::new(0.49671e0) * t24292 + F::new(0.82785e-1) * t24295 - t12542 - t12543 - F::new(0.82785e-1) * t24298 - F::cast_from(0.60384999999999999999e0_f64) * t24238 + F::new(0.181155e1) * t24246 + F::new(0.16504875e0) * t24313 + F::new(0.258925e1) * t24315 + F::new(0.19419375e1) * t24318 - F::cast_from(0.412621875e-1_f64) * t24320;
    t24406
}
