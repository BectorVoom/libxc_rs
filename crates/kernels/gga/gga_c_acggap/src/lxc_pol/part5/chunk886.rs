//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 886/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk886<F: Float>(t1089: F, t175: F, t3037: F, t3210: F, t495: F, t1008: F, t4518: F, t1163: F, t1165: F, t4210: F, t4289: F, t1137: F, t4769: F, t12936: F, t3655: F, t4417: F) -> (F, F, F, F, F) {
    let t15633 = t3210 * t1089 * t175 * t495 * t3037;
    let t15639 = t1008 * t4518;
    let t15653 = t1163 * t1165 * t4289 * t4210;
    let t15667 = t1137 * t4769;
    let t15671 = t12936 * t1165 * t4417 * t3655;
    (t15633, t15639, t15653, t15667, t15671)
}
