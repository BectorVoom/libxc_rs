//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 817/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk817<F: Float>(t11154: F, t5513: F, t122: F, t1293: F, t11141: F, t1613: F, t5585: F, t5584: F, t1608: F) -> (F, F, F) {
    let t22787 = t5513 * t11154;
    let t22790 = t1293 * t122;
    let t22791 = t22790 * t11141;
    let t22794 = t5585 * t1613;
    let t22795 = t5584 * t22794;
    let t22796 = t1608 * t22795;
    (t22787, t22791, t22796)
}
