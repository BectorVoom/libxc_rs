//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1429;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1430;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1431;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1432;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1433;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta398<F: Float>(t2435: F, t4477: F, t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F, t689: F, t213: F, t4469: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t10503: F, t10507: F, t10511: F, t10984: F, t10987: F, t2829: F, t4474: F, t887: F, t4533: F, t886: F, t2770: F, t2828: F, t10989: F, t10992: F, t10998: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11022: F, t2765: F, t4487: F, t4534: F, t865: F, t4321: F, t4470: F, t786: F, t789: F, t14939: F, t225: F, t779: F, t4322: F, t10513: F, t11026: F, t11030: F, t11037: F, t11040: F, t11045: F, t11051: F, t257: F, t2772: F, t14997: F, t2430: F, t4542: F, t10596: F, t10604: F, t10611: F, t14436: F, t14442: F, t14443: F, t14444: F, t14468: F, t14615: F, t14618: F, t14620: F, t14621: F, t14624: F, t14626: F, t14628: F, t14629: F, t1940: F, t198: F, t207: F, t2404: F, t2408: F, t4433: F, t4541: F, t765: F, t892: F, t9542: F, t14338: F, t14381: F, t14435: F, t2: F, t895: F, t580: F, t265: F, t22: F, t4567: F, t1610: F, t2875: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14998, t15003, t15004, t15006, t15010, t15011) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1428::<F>(t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469);
        let t15022 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1429::<F>(t1580, t2440, t2439, t1569, t2453, t2458, t10503, t10507, t10511, t10984, t10987, t14998, t15004, t15006, t15010, t15011, t2829, t4474, t887);
        let (t15030, t15038, t15044) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1430::<F>(t4533, t886, t2770, t1579, t2828, t10989, t10992, t10998, t11000, t11004, t11013, t11017, t11019, t11022, t2765, t4487, t4534, t865);
        let (t15047, t15050, t15054, t15062, t15063) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1431::<F>(t4321, t887, t689, t4470, t786, t789, t14939, t225, t4534, t779, t2435, t4322);
        let t15069 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1432::<F>(t10513, t11026, t11030, t11037, t11040, t11045, t11051, t15047, t15050, t15054, t15062, t15063, t1580, t213, t257, t2772, t4474);
        let (t15071, t15081) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1433::<F>(t14997, t15022, t15044, t15069, t2430, t4542, t10596, t10604, t10611, t14436, t14442, t14443, t14444, t14468, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t1940, t198, t207, t2404, t2408, t4433, t4541, t765, t892, t9542);
        let (t15083, t15093, t15094, t15096, t15098) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1434::<F>(t14338, t14381, t14435, t15081, t2, t895, t580, t265, t22, t4567, t1610, t2875);
    (t15003, t15030, t15038, t15071, t15083, t15093, t15094, t15096, t15098)
}
