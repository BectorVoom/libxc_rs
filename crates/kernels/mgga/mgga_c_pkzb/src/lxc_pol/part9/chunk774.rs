//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 774/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk774<F: Float>(t5484: F, t5490: F, t5493: F, t730: F, t1975: F, t257: F) -> (F, F, F) {
    let t5494 = t5490 * t5484 * t5493;
    let t5496 = F::cast_from(0.10254018858216406658e4_f64) * t730 * t5494;
    let t5498 = F::new(1.0) / t1975 / t257;
    (t5494, t5496, t5498)
}
