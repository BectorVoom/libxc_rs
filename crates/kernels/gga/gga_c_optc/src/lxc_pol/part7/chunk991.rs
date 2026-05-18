//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 991/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk991<F: Float>(t1784: F, t1793: F, t6433: F, t1757: F, t534: F, t539: F, t6340: F, t20814: F, t36: F, t88: F, t1785: F, t209: F, t6485: F) -> (F, F, F, F, F, F) {
    let t21874 = F::new(0.57894567559743977359e3) * t6433 * t1793 * t1784;
    let t21875 = t1784 * t1784;
    let t21878 = F::new(6.0) * t1757 * t21875 * t534;
    let t21879 = t539 * t6340;
    let t21880 = F::new(48.0) * t21879;
    let t21881 = F::new(1.0) / t20814;
    let t21884 = F::new(840.0) * t36 * t21881 * t88;
    let t21887 = F::new(0.14246666666666666667e0) * t209 * t6485 * t1785;
    (t21874, t21875, t21878, t21880, t21884, t21887)
}
