//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1259/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1259<F: Float>(t25071: F, t952: F, t3902: F, t866: F, t930: F, t7426: F, t8113: F, t2704: F, t7879: F, t25055: F, t953: F, t25077: F) -> (F, F, F, F, F, F) {
    let t26010 = t952 * t25071;
    let t26014 = t930 * t3902 * t866;
    let t26016 = t7426 * t8113;
    let t26019 = t2704 * t7879;
    let t26021 = t953 * t25055;
    let t26023 = t953 * t25077;
    (t26010, t26014, t26016, t26019, t26021, t26023)
}
