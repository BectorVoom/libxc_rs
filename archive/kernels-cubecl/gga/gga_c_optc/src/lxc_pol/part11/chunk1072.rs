//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1072/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1072<F: Float>(t1464: F, t8785: F, t1476: F, t8749: F, t8847: F, t8697: F, t1506: F, t26869: F, t8: F, t8425: F, t1121: F, t1508: F, t8528: F) -> (F, F, F, F, F, F, F) {
    let t34801 = t1464 * t8785;
    let t34813 = t1476 * t8749;
    let t34816 = t1464 * t8847;
    let t34829 = t1476 * t8697;
    let t35165 = t26869 * t1506;
    let t35363 = t8 * t8425;
    let t35379 = t1121 * t8528 * t1508;
    (t34801, t34813, t34816, t34829, t35165, t35363, t35379)
}
