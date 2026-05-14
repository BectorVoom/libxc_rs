//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 761/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk761<F: Float>(t22632: F, t5577: F, t5580: F, t397: F, t5539: F, t122: F, t1293: F, t1613: F, t5585: F, t5584: F, t1608: F, t22514: F, t72: F, t35: F, t53: F, t5612: F) -> (F, F, F, F, F, F, F) {
    let t22775 = t5577 * t22632 * t5580;
    let t22777 = t5539 * t397;
    let t22790 = t1293 * t122;
    let t22794 = t5585 * t1613;
    let t22795 = t5584 * t22794;
    let t22796 = t1608 * t22795;
    let t22797 = t22514 * t72;
    let t22798 = t35 * t53;
    let t22803 = t22632 * t5612;
    (t22775, t22777, t22790, t22796, t22797, t22798, t22803)
}
