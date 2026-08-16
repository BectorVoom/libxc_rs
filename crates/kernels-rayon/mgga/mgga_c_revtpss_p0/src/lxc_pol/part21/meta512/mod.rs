//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2141;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta512(t606: f64, t999: f64, t4578: f64, t3092: f64, t905: f64, t1045: f64, t15691: f64, t11774: f64, t11917: f64, t11924: f64, t11938: f64, t11952: f64, t11954: f64, t11956: f64, t11965: f64, t16078: f64, t16081: f64, t16084: f64, t16089: f64, t16091: f64, t16095: f64, t3115: f64, t3169: f64, t4820: f64, t1015: f64, t13312: f64, t1012: f64, t4573: f64, t11703: f64, t3188: f64, t4817: f64, t1011: f64, t11268: f64, t11714: f64, t11967: f64, t11972: f64, t11980: f64, t11989: f64, t12007: f64, t12010: f64, t1671: f64, t1675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16096, t16097, t16098, t16102, t16103, t16104, t16114) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2141(t606, t999, t4578, t3092, t905, t1045, t15691, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t3115);
        let (t16122, t16123, t16127, t16128, t16136) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2142(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
    (t16096, t16097, t16098, t16102, t16103, t16104, t16114, t16122, t16123, t16127, t16128, t16136)
}
