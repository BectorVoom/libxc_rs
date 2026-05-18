//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 657/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk657<F: Float>(t2469: F, t3832: F, t3753: F, t3758: F, t3766: F, t3770: F, t3773: F, t3782: F, t3785: F, t3790: F, t3653: F, t3800: F, t3801: F, t3802: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3834 = F::new(2.0) * t2469 * t3832;
    let t3835 = F::new(0.20240885416666666668e-4) * t3753;
    let t3836 = F::new(0.34752370105806885418e-3) * t3758;
    let t3838 = F::new(0.21720231316129303386e-4) * t3766;
    let t3839 = F::new(0.2318836277704281739e-4) * t3770;
    let t3840 = F::new(0.33764099580923002116e-6) * t3773;
    let t3842 = F::new(0.12290803273518880209e-8) * t3782;
    let t3843 = F::new(0.10551281119038438161e-7) * t3785;
    let t3844 = F::new(0.33147827249531850013e-7) * t3790;
    let t3903 = t3800 - t3801 - t3802 + F::new(0.5431140175846100239e-5) * t3653;
    (t3834, t3835, t3836, t3838, t3839, t3840, t3842, t3843, t3844, t3903)
}
