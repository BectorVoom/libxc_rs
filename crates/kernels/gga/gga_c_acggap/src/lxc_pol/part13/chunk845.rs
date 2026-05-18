//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 845/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk845<F: Float>(t1524: F, t157: F, t406: F, t1444: F, t372: F, t1449: F, t322: F, t1175: F, t1410: F, t1460: F, t513: F, t930: F) -> (F, F, F, F, F, F) {
    let t20972 = t1524 * t406 * t157;
    let t20987 = t1444 * t372;
    let t20992 = t1449 * t322;
    let t21099 = t1175 * t1410;
    let t21118 = t1460 * t322;
    let t21128 = t930 * t513;
    (t20972, t20987, t20992, t21099, t21118, t21128)
}
