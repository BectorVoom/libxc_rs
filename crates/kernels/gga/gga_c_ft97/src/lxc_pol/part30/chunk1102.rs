//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1102/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1102<F: Float>(t6260: F, t7021: F, t1486: F, t193: F, t2781: F, t4226: F, t6308: F, t7611: F, t852: F, t152767: F, t7512: F, t7638: F, t7641: F) -> (F, F, F, F) {
    let t152780 = t6260 * t7021;
    let t152783 = t1486 * t193 * t2781 * t152780;
    let t152788 = t6308 * t193 * t852 * t7611 * t4226;
    let t152792 = t7638 * t7512 * t7641 * t152767;
    (t152780, t152783, t152788, t152792)
}
