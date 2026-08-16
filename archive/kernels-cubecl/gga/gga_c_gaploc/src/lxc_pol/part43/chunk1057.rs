//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1057/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1057<F: Float>(t43603: F, t43604: F, t43605: F, t43606: F, t43607: F, t43617: F, t43619: F, t43627: F, t43630: F, t43636: F, t43640: F, t43645: F, t43646: F, t43648: F, t47255: F, t47261: F, t47263: F, t47267: F, t47274: F, t47275: F) -> F {
    let t51115 = -t43603 - t43604 - t43605 + t43606 - t43607 - F::cast_from(0.92023022289409799224e1_f64) * t47255 - t47261 - F::cast_from(0.18404604457881959845e2_f64) * t47263 - F::cast_from(0.50050685932590597338e1_f64) * t47267 + t47274 + F::cast_from(0.14300195980740170668e1_f64) * t47275 + t43617 + t43619 + t43627 + t43630 + t43636 + t43640 + t43645 + F::cast_from(0.89376224879626066674e-1_f64) * t43646 - t43648;
    t51115
}
