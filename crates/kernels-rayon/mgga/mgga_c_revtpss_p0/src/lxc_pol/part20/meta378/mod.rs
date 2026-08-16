//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1372;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta378(t10871: f64, t40262: f64, t14917: f64, t2475: f64, t2661: f64, t2662: f64, t836: f64, t2749: f64, t40378: f64, t2430: f64, t853: f64, t837: f64, t10638: f64, t125: f64, t124: f64, t2645: f64, t14686: f64, t14931: f64, t4366: f64, t2722: f64, t10777: f64, t10779: f64, t2682: f64, t820: f64, t823: f64, t2751: f64, t10764: f64, t10797: f64, t10870: f64, t14547: f64, t14785: f64, t14894: f64, t2721: f64, t2724: f64, t2745: f64, t2747: f64, t40263: f64, t40523: f64, t40526: f64, t40529: f64, t40532: f64, t40535: f64, t4362: f64, t4364: f64, t827: f64, t828: f64, t10886: f64, t808: f64, t10292: f64, t65: f64, t235: f64, t2710: f64, t826: f64, t225: f64, t785: f64, t2737: f64, t2694: f64, t9789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40537, t40549, t40553, t40555, t40558) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370(t10871, t40262, t14917, t2475, t2661, t2662, t836, t2749, t40378, t2430, t853, t837);
        let (t40560, t40569, t40578, t40581, t40583, t40586) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371(t2430, t836, t10638, t125, t124, t2645, t14686, t14931, t4366, t2722, t10777, t10779, t2749);
        let t40596 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1372(t2682, t820, t823, t2751, t10764, t10797, t10870, t14547, t14785, t14894, t2721, t2724, t2745, t2747, t2749, t40263, t40523, t40526, t40529, t40532, t40535, t40537, t40549, t40553, t40558, t40560, t40569, t40581, t40586, t4362, t4364, t827, t828, t837);
        let (t40600, t40604, t40607, t40609, t40611, t40625) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1373(t10886, t40555, t808, t10292, t65, t235, t2710, t826, t225, t785, t2737, t2694, t9789);
    (t40537, t40569, t40578, t40583, t40596, t40600, t40604, t40607, t40609, t40611, t40625)
}
