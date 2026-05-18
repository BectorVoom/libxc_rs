//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1054/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1054<F: Float>(t11767: F, t11770: F, t11779: F, t11782: F, t11785: F, t11787: F, t11792: F, t11796: F, t11800: F, t11806: F, t11809: F, t11811: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12204 = F::new(0.34752370105806885418e-3) * t11767;
    let t12205 = F::new(0.1422820120100248667e-7) * t11770;
    let t12208 = F::new(0.16908181191593721013e-5) * t11779;
    let t12209 = F::new(0.24760339692676868218e-5) * t11782;
    let t12210 = F::new(0.10551281119038438161e-7) * t11785;
    let t12211 = F::new(0.10551281119038438161e-7) * t11787;
    let t12213 = F::new(0.34752370105806885418e-3) * t11792;
    let t12214 = F::new(0.51491428373437201895e-5) * t11796;
    let t12215 = F::new(0.21720231316129303386e-4) * t11800;
    let t12216 = F::new(0.24581606547037760418e-8) * t11806;
    let t12217 = F::new(0.35170937063461460537e-8) * t11809;
    let t12218 = F::new(0.33147827249531850013e-7) * t11811;
    (t12204, t12205, t12208, t12209, t12210, t12211, t12213, t12214, t12215, t12216, t12217, t12218)
}
