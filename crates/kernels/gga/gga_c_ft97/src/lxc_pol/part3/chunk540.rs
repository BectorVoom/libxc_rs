//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 540/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk540<F: Float>(t1073: F, t2266: F, t925: F, t2271: F, t4417: F, t72: F, t4431: F, t632: F, t2281: F, t637: F, t2289: F, t3042: F, t4456: F, t4460: F, t4464: F, t4680: F, t4683: F) -> (F, F, F, F, F, F) {
    let t4861 = t2266 * t925 * t1073;
    let t4865 = t72 * t2271 * t4417;
    let t4869 = t72 * t632 * t4431;
    let t4872 = t1073 * t1073;
    let t4874 = t637 * t2281 * t4872;
    let t4883 = -0.117377e0 * t4680 + 0.234754e0 * t4683 + t2289 + 0.9628722222222222222e-1 * t3042 - 0.9628722222222222222e-1 * t4456 + 0.28886166666666666666e0 * t4460 - 0.14443083333333333333e0 * t4464;
    (t4861, t4865, t4869, t4872, t4874, t4883)
}
