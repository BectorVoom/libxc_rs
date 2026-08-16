//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 981/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk981<F: Float>(t40: F, t4100: F, t4102: F, t185: F, t5392: F, t2658: F, t1484: F, t4310: F, t1462: F, t4205: F, t2433: F, t5398: F, t73: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t5497 = F::cast_from(2.0_f64) * t4100;
    let t5498 = F::cast_from(8.0_f64) * t4102;
    let t5499 = t185 * t5392;
    let t5501 = F::cast_from(12.0_f64) * t2658 * t5499;
    let t5502 = t4310 * t1484;
    let t5506 = F::cast_from(8.0_f64) * t4205 * t1462;
    let t5512 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2433 * t5392 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t5398);
    (t5497, t5498, t5499, t5501, t5502, t5506, t5512)
}
