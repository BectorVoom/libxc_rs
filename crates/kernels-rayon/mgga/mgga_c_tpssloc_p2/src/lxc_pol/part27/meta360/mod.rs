//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1483;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1484;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta360(t2633: f64, t4180: f64, t4181: f64, t13029: f64, t225: f64, t237: f64, t2697: f64, t4261: f64, t12971: f64, t820: f64, t847: f64, t9645: f64, t1484: f64, t828: f64, t2647: f64, t1516: f64, t9993: f64, t2696: f64, t4166: f64, t849: f64, t13176: f64, t842: f64, t9601: f64, t10012: f64, t10014: f64, t10026: f64, t10029: f64, t10030: f64, t10036: f64, t10038: f64, t249: f64, t2623: f64, t2643: f64, t2703: f64, t2707: f64, t4172: f64, t4178: f64, t843: f64, t9990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13333, t13336, t13337, t13345, t13347, t13350) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1483(t2633, t4180, t4181, t13029, t225, t237, t2697, t4261, t12971, t820, t847, t9645);
        let (t13351, t13352, t13353, t13359, t13362, t13365, t13368) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1484(t1484, t828, t2647, t13350, t1516, t9993, t2696, t4166, t849, t13176, t842, t9601);
        let t13375 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1485(t10012, t10014, t10026, t10029, t10030, t10036, t10038, t13333, t13337, t13345, t13347, t13353, t13359, t13362, t13365, t13368, t1516, t249, t2623, t2643, t2703, t2707, t4172, t4178, t4261, t843, t849, t9990);
    (t13333, t13336, t13347, t13351, t13352, t13353, t13375)
}
