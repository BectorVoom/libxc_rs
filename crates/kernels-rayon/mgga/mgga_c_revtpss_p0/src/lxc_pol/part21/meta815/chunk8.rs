//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2993/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2993(t11262: f64, t3127: f64, t4874: f64, t11631: f64, t12116: f64, t15584: f64, t15906: f64, t1592: f64, t15968: f64, t16048: f64, t16081: f64, t16147: f64, t3092: f64, t3154: f64, t42550: f64, t42833: f64, t42883: f64, t42886: f64, t42889: f64, t42892: f64, t43069: f64, t4583: f64, t4786: f64, t4892: f64, t4896: f64) -> f64 {
    let t54599 = t3127 * t11262 * t4874;
    let t54622 = 0.45732285992607719436e-2_f64 * t42833 + 0.85748036236139473944e-3_f64 * t43069 * t15584 * t16147 * t4786 + 0.95275595817932748826e-4_f64 * t54599 + 0.85748036236139473944e-3_f64 * t4892 * t3092 * t4583 * t15968 + 0.85748036236139473944e-3_f64 * t16081 * t3092 * t1592 * t42550 * t11631 - 0.85748036236139473944e-3_f64 * t15906 * t3092 * t1592 * t42550 * t3154 + 0.14481890564325777822e-1_f64 * t42883 - 0.57165357490759649295e-3_f64 * t42886 + 0.47637797908966374414e-3_f64 * t42889 - 0.47637797908966374413e-3_f64 * t42892 - 0.13719685797782315831e-1_f64 * t12116 * t16048 * t4896;
    t54622
}
