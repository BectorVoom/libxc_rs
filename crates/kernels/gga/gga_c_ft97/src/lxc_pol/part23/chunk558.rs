//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 558/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk558<F: Float>(t7062: F, t852: F, t193: F, t6308: F, t1091: F, t2665: F, t6318: F, t6317: F, t2781: F, t7036: F, t1486: F, t7021: F, t6334: F, t992: F, t446: F, t1212: F, t6222: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7063 = t852 * t7062;
    let t7065 = t6308 * t193 * t7063;
    let t7068 = t2665 * t6318 * t1091;
    let t7069 = t6317 * t7068;
    let t7071 = t2781 * t7036;
    let t7073 = t1486 * t193 * t7071;
    let t7075 = t852 * t7021;
    let t7077 = t1486 * t193 * t7075;
    let t7079 = t6334 * t992;
    let t7080 = t2665 * t7079;
    let t7081 = t446 * t7080;
    let t7083 = t6222 * t1212;
    (t7063, t7065, t7068, t7069, t7071, t7073, t7075, t7077, t7080, t7081, t7083)
}
