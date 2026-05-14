//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 843/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk843<F: Float>(t1882: F, t33738: F, t33489: F, t761: F, t7548: F, t8232: F, t33683: F, t7484: F, t2492: F, t7536: F, t7555: F, t33617: F, t33638: F, t737: F, t33605: F, t7508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t141914 = t1882 * t33738;
    let t141916 = t761 * t33489;
    let t141942 = 8.0 / 27.0 * t8232 * t7548;
    let t141947 = t1882 * t33683;
    let t141989 = t761 * t7484;
    let t141997 = t2492 * t7536;
    let t142002 = 4.0 / 27.0 * t8232 * t7555;
    let t142009 = t1882 * t33617;
    let t142020 = t1882 * t33638;
    let t142030 = t737 * t7536;
    let t142058 = t1882 * t33605;
    let t142074 = 4.0 / 27.0 * t8232 * t7508;
    (t141914, t141916, t141942, t141947, t141989, t141997, t142002, t142009, t142020, t142030, t142058, t142074)
}
