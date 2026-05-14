//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 920/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk920<F: Float>(t3372: F, t5124: F, t14223: F, t5152: F, t3382: F, t4335: F, t3409: F, t4316: F, t1008: F, t4932: F, t5003: F, t12747: F, t1549: F, t1554: F, t1558: F, t3431: F, t4291: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17070 = t3372 * t5124;
    let t17072 = t14223 * t5152;
    let t17085 = t3382 * t4335;
    let t17088 = t3409 * t4316;
    let t17090 = t1008 * t4932;
    let t17092 = t1008 * t5003;
    let t17105 = t12747 * t1549;
    let t17107 = t12747 * t1554;
    let t17109 = t12747 * t1558;
    let t17111 = t3431 * t4291;
    (t17070, t17072, t17085, t17088, t17090, t17092, t17105, t17107, t17109, t17111)
}
