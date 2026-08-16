//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1493;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta405(t17: f64, t19573: f64, t6320: f64, t750: f64, t1388: f64, t1799: f64, t15877: f64, t11979: f64, t15890: f64, t15895: f64, t588: f64, t6328: f64, t592: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t3918: f64, t3919: f64, t5122: f64, t5126: f64, t5161: f64, t5187: f64, t5308: f64, t6347: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1493(t17, t19573, t6320, t750, t1388, t1799, t15877, t11979, t15890, t15895, t588, t6328);
        let (t19592, t19594, t19595) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1494(t19591, t592, t6328, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19577, t19581, t19588, t19589, t19590, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t6347, t9457, t9476, t9484);
    (t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19592, t19594, t19595)
}
