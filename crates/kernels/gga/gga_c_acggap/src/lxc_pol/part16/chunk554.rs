//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 554/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk554<F: Float>(t181: F, t315: F, t1219: F, t556: F, t871: F, t1662: F, t814: F, t467: F, t495: F, t3993: F, t2618: F, t1690: F, t2861: F, t1694: F, t886: F, t2868: F, t821: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5384 = t315 * t181;
    let t5385 = t1219 * t556;
    let t5386 = t5385 * t871;
    let t5388 = 0.26341796731742046394e1 * t5384 * t5386;
    let t5399 = t1662 * t814;
    let t5439 = t495 * t467;
    let t5443 = 0.21687162600603479684e-1 * t3993;
    let t5444 = 0.10843581300301739842e-1 * t2618;
    let t5445 = t2861 * t1690;
    let t5450 = t886 * t1694;
    let t5455 = -2.0 * t821 - 6.0 * t2868;
    (t5386, t5388, t5399, t5439, t5443, t5444, t5445, t5450, t5455)
}
