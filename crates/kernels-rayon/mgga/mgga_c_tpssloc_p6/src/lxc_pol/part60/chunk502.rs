//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 502/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk502(t457: f64, t6144: f64, t460: f64, t974: f64, t1174: f64, t1710: f64, t1717: f64, t3430: f64, t3447: f64, t463: f64, t4887: f64, t4889: f64, t4897: f64, t4917: f64, t6109: f64, t6120: f64, t6123: f64, t6127: f64, t6131: f64, t6141: f64) -> (f64, f64) {
    let t6145 = t457 * t6144;
    let t6146 = t6145 * t460;
    let t6147 = t974 * t6146;
    let t6150 = 0.81481481481481481481e-2_f64 * t6109 * t463 - 0.14814814814814814814e-2_f64 * t4887 + 0.14814814814814814814e-2_f64 * t4889 * t1710 + 0.44444444444444444444e-2_f64 * t4889 * t1717 - t3430 - 0.18518518518518518518e-3_f64 * t4897 - 0.55555555555555555554e-3_f64 * t4917 + 0.37037037037037037036e-3_f64 * t1174 * t6120 + 0.55555555555555555554e-3_f64 * t3447 * t6123 - 0.55555555555555555554e-3_f64 * t1174 * t6127 - 0.27777777777777777777e-3_f64 * t1174 * t6131 - 0.83333333333333333332e-3_f64 * t1174 * t6141 - 0.83333333333333333332e-3_f64 * t1174 * t6147;
    (t6146, t6150)
}
