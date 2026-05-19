//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 953/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk953<F: Float>(t457: F, t9091: F, t8936: F, t914: F, t1122: F, t430: F, t3126: F, t4356: F, t4463: F, t8193: F, t8914: F, t438: F, t8196: F) -> (F, F, F, F, F, F, F) {
    let t9093 = F::cast_from(0.32196894406625029092e-1_f64) * t457 * t9091;
    let t9094 = t914 * t8936;
    let t9097 = t430 * t1122;
    let t9098 = t4356 * t3126;
    let t9099 = t9097 * t9098;
    let t9102 = t4463 * t8193;
    let t9103 = t430 * t8914;
    let t9104 = t8196 * t438;
    (t9093, t9094, t9097, t9099, t9102, t9103, t9104)
}
