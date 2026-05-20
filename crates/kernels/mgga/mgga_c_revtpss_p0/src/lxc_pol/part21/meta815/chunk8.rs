//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2993/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2993<F: Float>(t11262: F, t3127: F, t4874: F, t11631: F, t12116: F, t15584: F, t15906: F, t1592: F, t15968: F, t16048: F, t16081: F, t16147: F, t3092: F, t3154: F, t42550: F, t42833: F, t42883: F, t42886: F, t42889: F, t42892: F, t43069: F, t4583: F, t4786: F, t4892: F, t4896: F) -> F {
    let t54599 = t3127 * t11262 * t4874;
    let t54622 = F::cast_from(0.45732285992607719436e-2_f64) * t42833 + F::cast_from(0.85748036236139473944e-3_f64) * t43069 * t15584 * t16147 * t4786 + F::cast_from(0.95275595817932748826e-4_f64) * t54599 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t3092 * t4583 * t15968 + F::cast_from(0.85748036236139473944e-3_f64) * t16081 * t3092 * t1592 * t42550 * t11631 - F::cast_from(0.85748036236139473944e-3_f64) * t15906 * t3092 * t1592 * t42550 * t3154 + F::cast_from(0.14481890564325777822e-1_f64) * t42883 - F::cast_from(0.57165357490759649295e-3_f64) * t42886 + F::cast_from(0.47637797908966374414e-3_f64) * t42889 - F::cast_from(0.47637797908966374413e-3_f64) * t42892 - F::cast_from(0.13719685797782315831e-1_f64) * t12116 * t16048 * t4896;
    t54622
}
