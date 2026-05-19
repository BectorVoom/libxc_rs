//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 961/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk961<F: Float>(t10704: F, t7064: F, t3440: F, t7137: F, t3420: F, t8469: F, t935: F) -> (F, F, F, F) {
    let t10705 = t7064 * t10704;
    let t10706 = F::cast_from(0.32043859292259267849e-3_f64) * t10705;
    let t10708 = F::cast_from(0.30762104920568897135e-1_f64) * t7137 * t3440;
    let t10710 = F::cast_from(0.10254034973522965712e-1_f64) * t7137 * t3420;
    let t10713 = t8469 * t935;
    (t10706, t10708, t10710, t10713)
}
