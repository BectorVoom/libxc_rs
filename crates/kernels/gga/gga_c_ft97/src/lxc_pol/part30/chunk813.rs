//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 813/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk813<F: Float>(t2492: F, t6154: F, t6061: F, t761: F, t24737: F, t53891: F, t229: F, t2917: F, t2842: F, t6347: F, t6260: F, t870: F, t848: F, t2770: F, t6353: F, t2404: F, t2781: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97777 = t2492 * t6154;
    let t97810 = t761 * t6061;
    let t98123 = t53891 * t24737;
    let t98545 = t229 * t2917;
    let t98724 = t6347 * t2842;
    let t98899 = t870 * t6260;
    let t99034 = t848 * t6347;
    let t99186 = t2770 * t6347;
    let t99238 = t2770 * t6353;
    let t99391 = t2404 * t2781;
    (t97777, t97810, t98123, t98545, t98724, t98899, t99034, t99186, t99238, t99391)
}
