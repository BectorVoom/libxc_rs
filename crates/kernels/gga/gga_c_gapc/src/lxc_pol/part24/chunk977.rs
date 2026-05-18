//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 977/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk977<F: Float>(t11808: F, t9419: F, t11784: F, t3789: F, t190: F, t932: F, t11449: F, t11804: F, t7735: F, t11781: F, t3375: F, t1084: F, t11508: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11809 = t11808 * t9419;
    let t11811 = t11784 * t3789;
    let t11813 = t932 * t190;
    let t11814 = t11813 * t11449;
    let t11815 = t11804 * t7735;
    let t11816 = t11814 * t11815;
    let t11818 = t11808 * t3789;
    let t11820 = t11781 * t3375;
    let t11822 = t1084 * t11508;
    (t11809, t11811, t11813, t11814, t11815, t11816, t11818, t11820, t11822)
}
