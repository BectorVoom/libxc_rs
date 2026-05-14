//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1372/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1372<F: Float>(t2743: F, t28842: F, t21409: F, t22102: F, t22105: F, t22107: F, t26498: F, t26500: F, t26504: F, t26513: F, t26515: F, t26518: F, t2769: F, t28615: F, t3137: F, t7761: F, t8972: F) -> (F,) {
    let t33518 = t2743 * t28842;
    let t33525 = -0.2025780996e0 * t8972 * t2769 - 0.2025780996e0 * t3137 * t7761 + 0.4051561992e0 * t33518 - t21409 + 0.34222787939297257218e3 * t26498 - t22102 + t22105 + t22107 - 60.0 * t28615 - 0.48796115851357829289e-1 * t26500 + 0.31580407562227089519e2 * t26504 + t26513 - 0.93505639170679904296e3 * t26515 - t26518;
    (t33525,)
}
