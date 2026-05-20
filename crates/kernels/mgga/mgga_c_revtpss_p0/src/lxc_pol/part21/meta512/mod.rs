//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2141;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta512<F: Float>(t606: F, t999: F, t4578: F, t3092: F, t905: F, t1045: F, t15691: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t16078: F, t16081: F, t16084: F, t16089: F, t16091: F, t16095: F, t3115: F, t3169: F, t4820: F, t1015: F, t13312: F, t1012: F, t4573: F, t11703: F, t3188: F, t4817: F, t1011: F, t11268: F, t11714: F, t11967: F, t11972: F, t11980: F, t11989: F, t12007: F, t12010: F, t1671: F, t1675: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16096, t16097, t16098, t16102, t16103, t16104, t16114) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2141::<F>(t606, t999, t4578, t3092, t905, t1045, t15691, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t3115);
        let (t16122, t16123, t16127, t16128, t16136) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2142::<F>(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
    (t16096, t16097, t16098, t16102, t16103, t16104, t16114, t16122, t16123, t16127, t16128, t16136)
}
