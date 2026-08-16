//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 737/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk737<F: Float>(t1680: F, t5439: F, t694: F, t3993: F, t2618: F, t1690: F, t2861: F, t1694: F, t886: F, t2868: F, t821: F) -> (F, F, F, F, F, F) {
    let t5441 = t694 * t1680 * t5439;
    let t5443 = F::cast_from(0.21687162600603479684e-1_f64) * t3993;
    let t5444 = F::cast_from(0.10843581300301739842e-1_f64) * t2618;
    let t5445 = t2861 * t1690;
    let t5450 = t886 * t1694;
    let t5455 = -F::cast_from(2.0_f64) * t821 - F::cast_from(6.0_f64) * t2868;
    (t5441, t5443, t5444, t5445, t5450, t5455)
}
