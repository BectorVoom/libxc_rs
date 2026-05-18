//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1058/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1058<F: Float>(t11792: F, t11796: F, t11800: F, t11806: F, t11809: F, t11811: F, t11816: F, t11818: F, t11820: F, t11823: F, t11829: F, t11832: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12213 = F::new(0.34752370105806885418e-3) * t11792;
    let t12214 = F::new(0.51491428373437201895e-5) * t11796;
    let t12215 = F::new(0.21720231316129303386e-4) * t11800;
    let t12216 = F::new(0.24581606547037760418e-8) * t11806;
    let t12217 = F::new(0.35170937063461460537e-8) * t11809;
    let t12218 = F::new(0.33147827249531850013e-7) * t11811;
    let t12219 = F::new(0.12290803273518880209e-8) * t11816;
    let t12220 = F::new(0.32042899674547455013e-6) * t11818;
    let t12221 = F::new(0.11254699860307667372e-6) * t11820;
    let t12222 = F::new(0.30353495895471971565e-6) * t11823;
    let t12224 = F::new(0.12290803273518880209e-8) * t11829;
    let t12225 = F::new(0.8193868849012586806e-9) * t11832;
    (t12213, t12214, t12215, t12216, t12217, t12218, t12219, t12220, t12221, t12222, t12224, t12225)
}
