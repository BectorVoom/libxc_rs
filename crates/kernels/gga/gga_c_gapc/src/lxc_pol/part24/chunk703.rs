//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 703/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk703<F: Float>(t1875: F, t9083: F, t4940: F, t8769: F, t5190: F, t116: F, t5294: F, t144: F, t3694: F, t3116: F, t3028: F, t677: F, t3021: F, t5075: F, t1043: F, t5979: F, t8820: F) -> (F, F, F, F, F, F, F, F) {
    let t9084 = t1875 * t9083;
    let t9085 = t9084 * t4940;
    let t9087 = t1875 * t8769;
    let t9088 = t9087 * t5190;
    let t9090 = t116 * t5294;
    let t9091 = t3694 * t144;
    let t9092 = t9091 * t3116;
    let t9093 = t9090 * t9092;
    let t9097 = t3028 * t677;
    let t9099 = t3021 * t5075;
    let t9100 = t1043 * t9099;
    let t9103 = t8820 * t144 * t5979;
    (t9085, t9088, t9092, t9093, t9097, t9099, t9100, t9103)
}
