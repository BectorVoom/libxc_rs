//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 982/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk982<F: Float>(t8: F, t8425: F, t1121: F, t1508: F, t8528: F, t1544: F, t7448: F, t1516: F, t8529: F, t11936: F, t1519: F, t3843: F, t1133: F, t1499: F, t8546: F, t8113: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35363 = t8 * t8425;
    let t35379 = t1121 * t8528 * t1508;
    let t35441 = t1544 * t7448;
    let t35453 = t1516 * t8529;
    let t35559 = t11936 * t7448;
    let t35576 = t3843 * t1519;
    let t35577 = t1133 * t35576;
    let t35643 = t1499 * t8546;
    let t35653 = t11936 * t8113;
    (t35363, t35379, t35441, t35453, t35559, t35576, t35577, t35643, t35653)
}
