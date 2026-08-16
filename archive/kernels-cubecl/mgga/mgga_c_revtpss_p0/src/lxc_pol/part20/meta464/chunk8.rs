//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1772/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1772<F: Float>(t10049: F, t1399: F, t47387: F, t47389: F, t47391: F, t47395: F, t47396: F, t47403: F, t47407: F, t47411: F, t47413: F, t47417: F, t820: F, t9912: F) -> F {
    let t47418 = -F::cast_from(0.21951497276451705328e-1_f64) * t47387 - F::cast_from(0.68293547082294194357e-1_f64) * t47389 + F::cast_from(0.39029762157531132075e-2_f64) * t47391 - t47395 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t47396 * t1399 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t10049 * t9912 + F::cast_from(0.87805989105806821314e-1_f64) * t47403 + F::cast_from(0.65854491829355115985e-1_f64) * t47407 - F::cast_from(0.13170898365871023197e0_f64) * t47411 - F::cast_from(0.7805952431506226415e-2_f64) * t47413 - t47417;
    t47418
}
