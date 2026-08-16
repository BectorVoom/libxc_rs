//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2561/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2561<F: Float>(t11629: F, t4869: F, t14967: F, t3411: F, t51474: F, t51476: F, t51478: F, t51480: F, t51482: F, t51485: F, t51549: F, t51593: F, t51831: F) -> (F, F, F) {
    let t51833 = F::cast_from(0.35089341735807877242e1_f64) * t4869 * t11629;
    let t51835 = F::cast_from(0.10389515463408878255e3_f64) * t3411 * t14967;
    let t51836 = t51474 - t51476 + t51478 - t51480 + t51482 - t51485 - t51831 + t51833 - t51549 - t51593 - t51835;
    (t51833, t51835, t51836)
}
