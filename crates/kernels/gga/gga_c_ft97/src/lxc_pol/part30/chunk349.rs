//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 349/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk349<F: Float>(t6154: F, t766: F, t242: F, t1451: F, t1882: F, t1449: F, t761: F, t684: F, t2606: F, t713: F) -> (F, F, F, F, F, F) {
    let t6155 = t6154 * t766;
    let t6156 = t242 * t6155;
    let t6160 = t1882 * t1451 / 9.0;
    let t6161 = t761 * t1449;
    let t6162 = t6161 * t684;
    let t6163 = t2606 * t6162;
    let t6166 = t1449 * t713;
    (t6156, t6160, t6161, t6162, t6163, t6166)
}
