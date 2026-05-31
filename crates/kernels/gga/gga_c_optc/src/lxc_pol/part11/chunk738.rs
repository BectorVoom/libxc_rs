//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 738/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk738<F: Float>(t1145: F, t454: F, t1: F, t3107: F, t1781: F, t321: F, t429: F, t457: F, t4463: F, t8193: F, t438: F, t8196: F) -> (F, F, F, F, F, F, F, F) {
    let t8995 = t1145 * t1145;
    let t8996 = F::cast_from(1.0_f64) / t8995;
    let t8997 = t454 * t8996;
    let t9073 = t3107 * t1;
    let t9091 = t321 * t1781 * t429;
    let t9093 = F::cast_from(0.32196894406625029092e-1_f64) * t457 * t9091;
    let t9102 = t4463 * t8193;
    let t9104 = t8196 * t438;
    (t8995, t8996, t8997, t9073, t9091, t9093, t9102, t9104)
}
