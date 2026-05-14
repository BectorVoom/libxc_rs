//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 615/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk615<F: Float>(t2469: F, t3832: F, t3753: F, t3758: F, t3766: F, t3770: F, t3773: F, t3782: F, t3785: F, t3790: F, t3653: F, t3800: F, t3801: F, t3802: F, t209: F, t3692: F, t3704: F, t3715: F, t3719: F, t3811: F, t3812: F, t3813: F, t3814: F, t3815: F, t3816: F, t3819: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3834 = 2.0 * t2469 * t3832;
    let t3835 = 0.20240885416666666668e-4 * t3753;
    let t3836 = 0.34752370105806885418e-3 * t3758;
    let t3838 = 0.21720231316129303386e-4 * t3766;
    let t3839 = 0.2318836277704281739e-4 * t3770;
    let t3840 = 0.33764099580923002116e-6 * t3773;
    let t3842 = 0.12290803273518880209e-8 * t3782;
    let t3843 = 0.10551281119038438161e-7 * t3785;
    let t3844 = 0.33147827249531850013e-7 * t3790;
    let t3903 = t3800 - t3801 - t3802 + 0.5431140175846100239e-5 * t3653;
    let t3904 = t3903 * t209;
    let t3909 = t3811 - t3812 - t3813 + t3814 - t3815 - t3816 + 0.57970906942607043475e-5 * t3692 - 0.49166375783284505216e-8 * t3704 + t3819 + 0.6629778687778673199e-7 * t3715 - 0.90579542097823505428e-7 * t3719;
    (t3834, t3835, t3836, t3838, t3839, t3840, t3842, t3843, t3844, t3903, t3904, t3909)
}
