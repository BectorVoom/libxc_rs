//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1056/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1056<F: Float>(t12176: F, t12190: F, t338: F, t11728: F, t11731: F, t11734: F, t11737: F, t11739: F, t11743: F, t11746: F, t11750: F, t11765: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12191 = t12176 + t12190;
    let t12192 = t12191 * t338;
    let t12193 = F::cast_from(0.21720231316129303386e-4_f64) * t11728;
    let t12194 = F::cast_from(0.21720231316129303386e-4_f64) * t11731;
    let t12195 = F::cast_from(0.2318836277704281739e-4_f64) * t11734;
    let t12196 = F::cast_from(0.12290803273518880209e-7_f64) * t11737;
    let t12197 = F::cast_from(0.16217772716043213195e-2_f64) * t11739;
    let t12198 = F::cast_from(0.21720231316129303386e-4_f64) * t11743;
    let t12199 = F::cast_from(0.5686343261418565457e-6_f64) * t11746;
    let t12200 = F::cast_from(0.2318836277704281739e-4_f64) * t11750;
    let t12203 = F::cast_from(0.34752370105806885418e-3_f64) * t11765;
    (t12191, t12192, t12193, t12194, t12195, t12196, t12197, t12198, t12199, t12200, t12203)
}
