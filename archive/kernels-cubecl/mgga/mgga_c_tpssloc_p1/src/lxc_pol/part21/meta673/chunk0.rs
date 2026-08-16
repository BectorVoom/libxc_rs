//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2478/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2478<F: Float>(t2281: F, t2333: F, t2359: F, t106: F, t9364: F, t35761: F, t35577: F, t111: F, t3931: F, t16546: F, t576: F, t16506: F, t580: F) -> (F, F, F, F, F, F, F, F) {
    let t45424 = t2281 * t2333;
    let t45426 = t2281 * t2359;
    let t45435 = F::cast_from(1.0_f64) / t9364 / t106;
    let t45460 = F::cast_from(1.0_f64) / t35761;
    let t45496 = F::cast_from(1.0_f64) / t35577;
    let t45560 = t3931 * t111;
    let t45584 = t576 * t16546;
    let t45588 = t16506 * t580;
    (t45424, t45426, t45435, t45460, t45496, t45560, t45584, t45588)
}
