//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 983/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk983<F: Float>(t15855: F, t8193: F, t1497: F, t9114: F, t15849: F, t1514: F, t8113: F, t9167: F, t15859: F, t1170: F, t1540: F, t3843: F, t1150: F, t1528: F, t3902: F, t1162: F, t1502: F) -> (F, F, F, F, F, F, F, F) {
    let t35724 = t15855 * t8193;
    let t35730 = t9114 * t1497 * t8193;
    let t35733 = t15849 * t8193;
    let t35745 = t9167 * t1514 * t8113;
    let t35748 = t15859 * t8113;
    let t35825 = t1170 * t3843 * t1540;
    let t35834 = t1150 * t3902 * t1528;
    let t35887 = t1162 * t3902 * t1502;
    (t35724, t35730, t35733, t35745, t35748, t35825, t35834, t35887)
}
