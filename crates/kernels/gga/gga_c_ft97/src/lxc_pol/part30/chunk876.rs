//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 876/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk876<F: Float>(t35858: F, t36001: F, t312: F, t6353: F, t7124: F, t1248: F, t34012: F, t1501: F, t28859: F, t1212: F, t1506: F, t6222: F) -> (F, F, F, F, F, F, F) {
    let t36002 = t35858 + t36001;
    let t36003 = t36002 * t312;
    let t36005 = t6353 * t7124;
    let t36007 = t34012 * t1248;
    let t36009 = t28859 * t1501;
    let t36011 = t1506 * t1212;
    let t36012 = t6222 * t36011;
    (t36002, t36003, t36005, t36007, t36009, t36011, t36012)
}
