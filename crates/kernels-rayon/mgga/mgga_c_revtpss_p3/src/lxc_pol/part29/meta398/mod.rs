//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1429;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1430;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1431;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1432;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1433;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta398(t2435: f64, t4477: f64, t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2444: f64, t689: f64, t213: f64, t4469: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t10987: f64, t2829: f64, t4474: f64, t887: f64, t4533: f64, t886: f64, t2770: f64, t2828: f64, t10989: f64, t10992: f64, t10998: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t2765: f64, t4487: f64, t4534: f64, t865: f64, t4321: f64, t4470: f64, t786: f64, t789: f64, t14939: f64, t225: f64, t779: f64, t4322: f64, t10513: f64, t11026: f64, t11030: f64, t11037: f64, t11040: f64, t11045: f64, t11051: f64, t257: f64, t2772: f64, t14997: f64, t2430: f64, t4542: f64, t10596: f64, t10604: f64, t10611: f64, t14436: f64, t14442: f64, t14443: f64, t14444: f64, t14468: f64, t14615: f64, t14618: f64, t14620: f64, t14621: f64, t14624: f64, t14626: f64, t14628: f64, t14629: f64, t1940: f64, t198: f64, t207: f64, t2404: f64, t2408: f64, t4433: f64, t4541: f64, t765: f64, t892: f64, t9542: f64, t14338: f64, t14381: f64, t14435: f64, t2: f64, t895: f64, t580: f64, t265: f64, t22: f64, t4567: f64, t1610: f64, t2875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14998, t15003, t15004, t15006, t15010, t15011) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1428(t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469);
        let t15022 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1429(t1580, t2440, t2439, t1569, t2453, t2458, t10503, t10507, t10511, t10984, t10987, t14998, t15004, t15006, t15010, t15011, t2829, t4474, t887);
        let (t15030, t15038, t15044) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1430(t4533, t886, t2770, t1579, t2828, t10989, t10992, t10998, t11000, t11004, t11013, t11017, t11019, t11022, t2765, t4487, t4534, t865);
        let (t15047, t15050, t15054, t15062, t15063) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1431(t4321, t887, t689, t4470, t786, t789, t14939, t225, t4534, t779, t2435, t4322);
        let t15069 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1432(t10513, t11026, t11030, t11037, t11040, t11045, t11051, t15047, t15050, t15054, t15062, t15063, t1580, t213, t257, t2772, t4474);
        let (t15071, t15081) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1433(t14997, t15022, t15044, t15069, t2430, t4542, t10596, t10604, t10611, t14436, t14442, t14443, t14444, t14468, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t1940, t198, t207, t2404, t2408, t4433, t4541, t765, t892, t9542);
        let (t15083, t15093, t15094, t15096, t15098) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1434(t14338, t14381, t14435, t15081, t2, t895, t580, t265, t22, t4567, t1610, t2875);
    (t15003, t15030, t15038, t15071, t15083, t15093, t15094, t15096, t15098)
}
