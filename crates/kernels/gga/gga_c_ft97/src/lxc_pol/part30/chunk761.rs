//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 761/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk761<F: Float>(t242: F, t33608: F, t684: F, t724: F, t7560: F, t10157: F, t265: F, t33302: F, t7440: F, t766: F, t2574: F, t762: F) -> (F, F, F, F, F) {
    let t33609 = t242 * t33608;
    let t33613 = t724 * t7560 * t684;
    let t33617 = t10157 * t265 * t33302;
    let t33620 = t7440 * t766;
    let t33622 = t2574 * t762 * t33620;
    (t33609, t33613, t33617, t33620, t33622)
}
