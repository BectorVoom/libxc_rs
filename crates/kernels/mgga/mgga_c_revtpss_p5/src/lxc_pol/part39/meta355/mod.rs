//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1214;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1215;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1216;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1217;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1218;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1219;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1220;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta355<F: Float>(t14519: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t4496: F, t1568: F, t836: F, t231: F, t2783: F, t2782: F, t10519: F, t10524: F, t10943: F, t14498: F, t14502: F, t14506: F, t14507: F, t14511: F, t14512: F, t14518: F, t4366: F, t4494: F, t4504: F, t4514: F, t837: F, t10867: F, t225: F, t213: F, t10871: F, t2722: F, t2777: F, t4518: F, t2439: F, t2470: F, t4499: F, t786: F, t2801: F, t10533: F, t10539: F, t10543: F, t10548: F, t10645: F, t10647: F, t10651: F, t10655: F, t2646: F, t2724: F, t2754: F, t4526: F, t820: F, t233: F, t4469: F, t869: F, t689: F, t2435: F, t4519: F, t1558: F, t2723: F, t10529: F, t72: F, t874: F, t2811: F, t2482: F, t122: F, t676: F, t879: F, t10443: F, t10552: F, t10554: F, t14312: F, t14313: F, t14315: F, t14317: F, t14324: F, t14327: F, t14329: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t10566: F, t10568: F, t14333: F, t14335: F, t14337: F, t14340: F, t14343: F, t14345: F, t14352: F, t14364: F, t14372: F, t14373: F, t14374: F, t14379: F, t14380: F, t9394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14522, t14525, t14533, t14537) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1214::<F>(t14519, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836, t231, t2783);
        let t14540 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1215::<F>(t14537, t2782, t10519, t10524, t10943, t14498, t14502, t14506, t14507, t14511, t14512, t14518, t14522, t14525, t14533, t4366, t4494, t4504, t4514, t837);
        let (t14546, t14547, t14558, t14564, t14567) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1216::<F>(t10867, t225, t213, t10871, t2722, t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783);
        let t14572 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1217::<F>(t14567, t786, t2801, t10533, t10539, t10543, t10548, t10645, t10647, t10651, t10655, t14546, t14547, t14558, t14564, t2646, t2724, t2754, t4494, t4504, t4514, t4526, t820);
        let (t14577, t14581, t14586, t14590, t14593) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1218::<F>(t233, t4469, t869, t689, t2435, t4519, t1558, t2723, t836, t10529, t2782, t72);
        let (t14596, t14603, t14605) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1219::<F>(t14593, t686, t874, t1558, t2811, t2482, t122, t2723, t72, t676, t836, t879);
        let (t14608, t14609) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1220::<F>(t14605, t2482, t2801, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t9278, t9308, t9316, t9329, t9333);
        let t14610 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1221::<F>(t10566, t10568, t14333, t14335, t14337, t14340, t14343, t14345, t14352, t14364, t14372, t14373, t14374, t14379, t14380, t9394);
    (t14540, t14547, t14572, t14577, t14581, t14586, t14590, t14596, t14603, t14608, t14609, t14610)
}
