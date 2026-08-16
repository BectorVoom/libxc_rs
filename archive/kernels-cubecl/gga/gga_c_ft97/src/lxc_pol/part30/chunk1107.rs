//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1107/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1107<F: Float>(t10248: F, t152844: F, t446: F, t193: F, t35972: F, t6308: F, t852: F, t856: F, t824: F, t1486: F, t2781: F, t4129: F, t7611: F) -> (F, F, F, F, F) {
    let t152849 = t446 * t10248 * t152844;
    let t152854 = t6308 * t193 * t852 * t35972 * t856;
    let t152856 = t35972 * t824;
    let t152859 = t1486 * t193 * t2781 * t152856;
    let t152861 = t7611 * t4129;
    (t152849, t152854, t152856, t152859, t152861)
}
