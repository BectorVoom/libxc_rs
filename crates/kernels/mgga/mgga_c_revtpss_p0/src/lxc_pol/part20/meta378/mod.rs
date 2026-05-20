//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1372;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta378<F: Float>(t10871: F, t40262: F, t14917: F, t2475: F, t2661: F, t2662: F, t836: F, t2749: F, t40378: F, t2430: F, t853: F, t837: F, t10638: F, t125: F, t124: F, t2645: F, t14686: F, t14931: F, t4366: F, t2722: F, t10777: F, t10779: F, t2682: F, t820: F, t823: F, t2751: F, t10764: F, t10797: F, t10870: F, t14547: F, t14785: F, t14894: F, t2721: F, t2724: F, t2745: F, t2747: F, t40263: F, t40523: F, t40526: F, t40529: F, t40532: F, t40535: F, t4362: F, t4364: F, t827: F, t828: F, t10886: F, t808: F, t10292: F, t65: F, t235: F, t2710: F, t826: F, t225: F, t785: F, t2737: F, t2694: F, t9789: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40537, t40549, t40553, t40555, t40558) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1370::<F>(t10871, t40262, t14917, t2475, t2661, t2662, t836, t2749, t40378, t2430, t853, t837);
        let (t40560, t40569, t40578, t40581, t40583, t40586) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371::<F>(t2430, t836, t10638, t125, t124, t2645, t14686, t14931, t4366, t2722, t10777, t10779, t2749);
        let t40596 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1372::<F>(t2682, t820, t823, t2751, t10764, t10797, t10870, t14547, t14785, t14894, t2721, t2724, t2745, t2747, t2749, t40263, t40523, t40526, t40529, t40532, t40535, t40537, t40549, t40553, t40558, t40560, t40569, t40581, t40586, t4362, t4364, t827, t828, t837);
        let (t40600, t40604, t40607, t40609, t40611, t40625) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1373::<F>(t10886, t40555, t808, t10292, t65, t235, t2710, t826, t225, t785, t2737, t2694, t9789);
    (t40537, t40569, t40578, t40583, t40596, t40600, t40604, t40607, t40609, t40611, t40625)
}
