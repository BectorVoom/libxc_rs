//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1390/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1390<F: Float>(t1726: F, t2798: F, t5228: F, t5943: F, t7666: F, t2788: F, t5893: F, t1759: F, t584: F, t7778: F, t5948: F, t7783: F, t5223: F, t21098: F, t4885: F, t959: F) -> (F, F, F, F, F, F, F, F) {
    let t26374 = t1726 * t2798 * t5228;
    let t26376 = t7666 * t5943;
    let t26378 = t2788 * t5893;
    let t26381 = t584 * t7778 * t1759;
    let t26382 = 0.1016176784e-1 * t26381;
    let t26384 = t584 * t7783 * t5948;
    let t26386 = t7666 * t5223;
    let t26388 = 96.0 * t21098;
    let t26389 = t4885 * t959;
    (t26374, t26376, t26378, t26382, t26384, t26386, t26388, t26389)
}
