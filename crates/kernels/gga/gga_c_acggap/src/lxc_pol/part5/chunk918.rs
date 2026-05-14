//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 918/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk918<F: Float>(t1181: F, t12936: F, t3044: F, t535: F, t16899: F, t4465: F, t1462: F, t3670: F, t4999: F, t952: F, t5108: F, t997: F, t1008: F, t5255: F, t10098: F, t1162: F) -> (F, F, F, F, F, F, F) {
    let t17000 = t12936 * t1181 * t535 * t3044;
    let t17002 = t16899 * t4465;
    let t17016 = t3670 * t1462;
    let t17018 = t952 * t4999;
    let t17020 = t997 * t5108;
    let t17029 = t1008 * t5255;
    let t17039 = t10098 * t1162;
    (t17000, t17002, t17016, t17018, t17020, t17029, t17039)
}
