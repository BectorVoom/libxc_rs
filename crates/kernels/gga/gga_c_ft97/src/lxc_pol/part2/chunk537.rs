//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 537/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk537<F: Float>(t713: F, t992: F, t2600: F, t2599: F, t766: F, t2607: F, t2606: F, t2360: F, t258: F, t505: F) -> (F, F, F, F, F, F) {
    let t3875 = t992 * t713;
    let t3876 = t2600 * t3875;
    let t3877 = t2599 * t3876;
    let t3880 = t992 * t766;
    let t3881 = t2607 * t3880;
    let t3882 = t2606 * t3881;
    let t3885 = t258 * t2360;
    let t3886 = t992 * t505;
    (t3876, t3877, t3881, t3882, t3885, t3886)
}
