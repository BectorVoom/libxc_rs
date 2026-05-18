//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 419/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk419<F: Float>(t1154: F, t2475: F, t1148: F, t1775: F, t2: F, t2486: F, t737: F, t1152: F, t458: F, t1131: F, t3688: F, t3710: F) -> (F, F, F, F, F, F, F, F) {
    let t3902 = t2475 * t1154;
    let t3908 = t1775 * t1148;
    let t3910 = t2486 * t2;
    let t3917 = t737 * t2;
    let t3925 = t458 * t1152;
    let t3930 = t2 * t1131;
    let t3942 = t3688 / F::new(27.0);
    let t3947 = t3710 / F::new(9.0);
    (t3902, t3908, t3910, t3917, t3925, t3930, t3942, t3947)
}
