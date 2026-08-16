//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1772/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1772(t10049: f64, t1399: f64, t47387: f64, t47389: f64, t47391: f64, t47395: f64, t47396: f64, t47403: f64, t47407: f64, t47411: f64, t47413: f64, t47417: f64, t820: f64, t9912: f64) -> f64 {
    let t47418 = -0.21951497276451705328e-1_f64 * t47387 - 0.68293547082294194357e-1_f64 * t47389 + 0.39029762157531132075e-2_f64 * t47391 - t47395 - 0.26341796731742046395e1_f64 * t820 * t47396 * t1399 + 0.15805078039045227836e2_f64 * t820 * t10049 * t9912 + 0.87805989105806821314e-1_f64 * t47403 + 0.65854491829355115985e-1_f64 * t47407 - 0.13170898365871023197e0_f64 * t47411 - 0.7805952431506226415e-2_f64 * t47413 - t47417;
    t47418
}
