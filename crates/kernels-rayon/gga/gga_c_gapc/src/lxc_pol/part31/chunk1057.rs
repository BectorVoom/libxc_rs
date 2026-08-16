//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1057/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1057(t11767: f64, t11770: f64, t11779: f64, t11782: f64, t11785: f64, t11787: f64, t11792: f64, t11796: f64, t11800: f64, t11806: f64, t11809: f64, t11811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12204 = 0.34752370105806885418e-3_f64 * t11767;
    let t12205 = 0.1422820120100248667e-7_f64 * t11770;
    let t12208 = 0.16908181191593721013e-5_f64 * t11779;
    let t12209 = 0.24760339692676868218e-5_f64 * t11782;
    let t12210 = 0.10551281119038438161e-7_f64 * t11785;
    let t12211 = 0.10551281119038438161e-7_f64 * t11787;
    let t12213 = 0.34752370105806885418e-3_f64 * t11792;
    let t12214 = 0.51491428373437201895e-5_f64 * t11796;
    let t12215 = 0.21720231316129303386e-4_f64 * t11800;
    let t12216 = 0.24581606547037760418e-8_f64 * t11806;
    let t12217 = 0.35170937063461460537e-8_f64 * t11809;
    let t12218 = 0.33147827249531850013e-7_f64 * t11811;
    (t12204, t12205, t12208, t12209, t12210, t12211, t12213, t12214, t12215, t12216, t12217, t12218)
}
