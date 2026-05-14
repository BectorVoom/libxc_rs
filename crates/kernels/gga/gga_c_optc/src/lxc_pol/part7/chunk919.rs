//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 919/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk919<F: Float>(t1784: F, t1793: F, t6433: F, t1757: F, t534: F, t539: F, t6340: F, t20814: F, t36: F, t88: F, t1785: F, t209: F, t6485: F, t110: F, t1789: F, t508: F, t6432: F, t6435: F) -> (F, F, F, F, F, F, F, F) {
    let t21874 = 0.57894567559743977359e3 * t6433 * t1793 * t1784;
    let t21875 = t1784 * t1784;
    let t21878 = 6.0 * t1757 * t21875 * t534;
    let t21879 = t539 * t6340;
    let t21880 = 48.0 * t21879;
    let t21881 = 1.0 / t20814;
    let t21884 = 840.0 * t36 * t21881 * t88;
    let t21887 = 0.14246666666666666667e0 * t209 * t6485 * t1785;
    let t21891 = 0.2291123905095794067e1 * t209 * t110 * t1789 * t1793;
    let t21895 = 0.68733717152873822009e1 * t209 * t508 * t6432 * t6435;
    (t21874, t21875, t21878, t21880, t21884, t21887, t21891, t21895)
}
