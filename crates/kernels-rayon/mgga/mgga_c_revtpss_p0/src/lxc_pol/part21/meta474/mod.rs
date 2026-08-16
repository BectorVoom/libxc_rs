//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2037;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2038;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2039;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta474(t4321: f64, t887: f64, t689: f64, t4470: f64, t786: f64, t789: f64, t14939: f64, t225: f64, t4534: f64, t779: f64, t2435: f64, t4322: f64, t10513: f64, t11026: f64, t11030: f64, t11037: f64, t11040: f64, t11045: f64, t11051: f64, t1580: f64, t213: f64, t257: f64, t2772: f64, t4474: f64, t14997: f64, t15022: f64, t15044: f64, t2430: f64, t4542: f64, t10596: f64, t10604: f64, t10611: f64, t14436: f64, t14442: f64, t14443: f64, t14444: f64, t14468: f64, t14615: f64, t14618: f64, t14620: f64, t14621: f64, t14624: f64, t14626: f64, t14628: f64, t14629: f64, t1940: f64, t198: f64, t207: f64, t2404: f64, t2408: f64, t4433: f64, t4541: f64, t765: f64, t892: f64, t9542: f64, t14338: f64, t14381: f64, t14435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15045, t15047, t15048, t15050, t15054, t15060, t15062, t15063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2037(t4321, t887, t689, t4470, t786, t789, t14939, t225, t4534, t779, t2435, t4322);
        let t15069 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2038(t10513, t11026, t11030, t11037, t11040, t11045, t11051, t15047, t15050, t15054, t15062, t15063, t1580, t213, t257, t2772, t4474);
        let (t15071, t15078, t15081) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2039(t14997, t15022, t15044, t15069, t2430, t4542, t10596, t10604, t10611, t14436, t14442, t14443, t14444, t14468, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t1940, t198, t207, t2404, t2408, t4433, t4541, t765, t892, t9542);
        let t15083 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2040(t14338, t14381, t14435, t15081);
    (t15045, t15048, t15054, t15060, t15071, t15078, t15083)
}
