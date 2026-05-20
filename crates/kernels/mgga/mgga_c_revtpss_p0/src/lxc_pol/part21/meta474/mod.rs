//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2037;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2038;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2039;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta474<F: Float>(t4321: F, t887: F, t689: F, t4470: F, t786: F, t789: F, t14939: F, t225: F, t4534: F, t779: F, t2435: F, t4322: F, t10513: F, t11026: F, t11030: F, t11037: F, t11040: F, t11045: F, t11051: F, t1580: F, t213: F, t257: F, t2772: F, t4474: F, t14997: F, t15022: F, t15044: F, t2430: F, t4542: F, t10596: F, t10604: F, t10611: F, t14436: F, t14442: F, t14443: F, t14444: F, t14468: F, t14615: F, t14618: F, t14620: F, t14621: F, t14624: F, t14626: F, t14628: F, t14629: F, t1940: F, t198: F, t207: F, t2404: F, t2408: F, t4433: F, t4541: F, t765: F, t892: F, t9542: F, t14338: F, t14381: F, t14435: F) -> (F, F, F, F, F, F, F) {
        let (t15045, t15047, t15048, t15050, t15054, t15060, t15062, t15063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2037::<F>(t4321, t887, t689, t4470, t786, t789, t14939, t225, t4534, t779, t2435, t4322);
        let t15069 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2038::<F>(t10513, t11026, t11030, t11037, t11040, t11045, t11051, t15047, t15050, t15054, t15062, t15063, t1580, t213, t257, t2772, t4474);
        let (t15071, t15078, t15081) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2039::<F>(t14997, t15022, t15044, t15069, t2430, t4542, t10596, t10604, t10611, t14436, t14442, t14443, t14444, t14468, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t1940, t198, t207, t2404, t2408, t4433, t4541, t765, t892, t9542);
        let t15083 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2040::<F>(t14338, t14381, t14435, t15081);
    (t15045, t15048, t15054, t15060, t15071, t15078, t15083)
}
