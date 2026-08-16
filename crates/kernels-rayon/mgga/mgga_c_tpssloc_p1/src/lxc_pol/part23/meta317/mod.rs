//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1075;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta317(t22055: f64, t3440: f64, t20234: f64, t3441: f64, t1177: f64, t21745: f64, t4900: f64, t15390: f64, t18469: f64, t18416: f64, t4904: f64, t18409: f64, t4919: f64, t18427: f64, t11547: f64, t11546: f64, t1174: f64, t15265: f64, t1710: f64, t1717: f64, t18321: f64, t22035: f64, t22041: f64, t22047: f64, t22052: f64, t3447: f64, t4889: f64, t6120: f64, t6141: f64, t6147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22056, t22059, t22060, t22063, t22066, t22069, t22072) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1075(t22055, t3440, t20234, t3441, t1177, t21745, t4900, t15390, t18469, t18416, t4904, t18409, t4919);
        let (t22075, t22081, t22082, t22085) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1076(t18427, t4919, t11547, t20234, t11546, t1174, t15265, t1710, t1717, t18321, t22035, t22041, t22047, t22052, t22056, t22060, t22063, t22066, t22069, t22072, t3447, t4889, t6120, t6141, t6147);
    (t22056, t22059, t22060, t22063, t22066, t22069, t22072, t22075, t22081, t22082, t22085)
}
