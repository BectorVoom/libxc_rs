//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 333/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk333<F: Float>(t1038: F, t1450: F, t1045: F, t1051: F, t1442: F, t25: F, t1043: F, t1050: F, t1444: F) -> (F, F, F, F, F) {
    let t1451 = t1038 * t1450;
    let t1454 = t1045 * t1450;
    let t1456 = t1051 * t1442;
    let t1457 = t25 * t1456;
    let t1459 = F::new(0.1898925e1) * t1451 - t1043 - F::cast_from(0.29896666666666666667e0_f64) * t1444 + F::new(0.3071625e0) * t1454 - t1050 - F::cast_from(0.82156666666666666667e-1_f64) * t1457;
    (t1451, t1454, t1456, t1457, t1459)
}
