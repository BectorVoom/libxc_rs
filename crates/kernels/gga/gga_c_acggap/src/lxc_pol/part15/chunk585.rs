//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 585/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk585<F: Float>(t1219: F, t556: F, t871: F, t5384: F, t1662: F, t814: F, t467: F, t495: F, t3993: F, t2618: F, t1690: F, t2861: F) -> (F, F, F, F, F, F, F) {
    let t5385 = t1219 * t556;
    let t5386 = t5385 * t871;
    let t5388 = F::cast_from(0.26341796731742046394e1_f64) * t5384 * t5386;
    let t5399 = t1662 * t814;
    let t5439 = t495 * t467;
    let t5443 = F::cast_from(0.21687162600603479684e-1_f64) * t3993;
    let t5444 = F::cast_from(0.10843581300301739842e-1_f64) * t2618;
    let t5445 = t2861 * t1690;
    (t5386, t5388, t5399, t5439, t5443, t5444, t5445)
}
