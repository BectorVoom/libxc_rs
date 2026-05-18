//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 287/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk287<F: Float>(t3902: F, t747: F, t91: F, t1148: F, t1775: F, t2: F, t2486: F, t3691: F, t2493: F, t3695: F, t737: F, t3700: F) -> (F, F, F, F, F) {
    let t3904 = t91 * t3902 * t747;
    let t3908 = t1775 * t1148;
    let t3910 = t2486 * t2;
    let t3911 = t3910 * t3691;
    let t3914 = t2493 * t3695;
    let t3917 = t737 * t2;
    let t3918 = t3917 * t3700;
    (t3904, t3908, t3911, t3914, t3918)
}
