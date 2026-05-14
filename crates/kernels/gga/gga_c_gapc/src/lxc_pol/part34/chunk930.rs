//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 930/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk930<F: Float>(t11731: F, t11734: F, t11737: F, t11739: F, t11743: F, t11746: F, t11750: F, t11765: F, t11767: F, t11770: F, t11779: F, t11782: F, t11785: F, t11787: F, t11756: F, t11762: F, t11773: F, t11776: F, t12193: F) -> (F,) {
    let t12194 = 0.21720231316129303386e-4 * t11731;
    let t12195 = 0.2318836277704281739e-4 * t11734;
    let t12196 = 0.12290803273518880209e-7 * t11737;
    let t12197 = 0.16217772716043213195e-2 * t11739;
    let t12198 = 0.21720231316129303386e-4 * t11743;
    let t12199 = 0.5686343261418565457e-6 * t11746;
    let t12200 = 0.2318836277704281739e-4 * t11750;
    let t12203 = 0.34752370105806885418e-3 * t11765;
    let t12204 = 0.34752370105806885418e-3 * t11767;
    let t12205 = 0.1422820120100248667e-7 * t11770;
    let t12208 = 0.16908181191593721013e-5 * t11779;
    let t12209 = 0.24760339692676868218e-5 * t11782;
    let t12210 = 0.10551281119038438161e-7 * t11785;
    let t12211 = 0.10551281119038438161e-7 * t11787;
    let t12212 = t12193 + t12194 - t12195 + t12196 + t12197 - t12198 - t12199 + t12200 - 0.252977417353824213e-7 * t11756 + 0.12228868272569444446e-4 * t11762 - t12203 - t12204 + t12205 + 0.12650553385416666668e-5 * t11773 + 0.12650553385416666668e-5 * t11776 + t12208 + t12209 + t12210 + t12211;
    (t12212,)
}
