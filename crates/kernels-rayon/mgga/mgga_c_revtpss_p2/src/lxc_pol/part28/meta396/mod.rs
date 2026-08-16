//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta396 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1493;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1494;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1495;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1496;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1497;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1498;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta396(t14502: f64, t231: f64, t2783: f64, t2782: f64, t251: f64, t4423: f64, t10073: f64, t4496: f64, t10542: f64, t4500: f64, t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t1568: f64, t836: f64, t10519: f64, t10524: f64, t10943: f64, t14498: f64, t4366: f64, t4494: f64, t4504: f64, t4514: f64, t837: f64, t10867: f64, t225: f64, t213: f64, t10871: f64, t2722: f64, t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64, t786: f64, t2801: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t2646: f64, t2724: f64, t2754: f64, t4526: f64, t820: f64, t233: f64, t4469: f64, t869: f64, t689: f64, t2435: f64, t4519: f64, t1558: f64, t2723: f64, t10529: f64, t874: f64, t2811: f64, t2482: f64, t122: f64, t676: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14506, t14507, t14511, t14512, t14518, t14519) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1493(t14502, t231, t2783, t2782, t251, t4423, t10073, t4496, t10542, t4500, t4424, t72);
        let (t14522, t14525, t14533, t14537) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1494(t14519, t686, t2798, t136, t1559, t2457, t10535, t10069, t4496, t1568, t836, t231, t2783);
        let t14540 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1495(t14537, t2782, t10519, t10524, t10943, t14498, t14502, t14506, t14507, t14511, t14512, t14518, t14522, t14525, t14533, t4366, t4494, t4504, t4514, t837);
        let (t14546, t14547, t14558, t14564, t14567) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1496(t10867, t225, t213, t10871, t2722, t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783);
        let t14572 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1497(t14567, t786, t2801, t10533, t10539, t10543, t10548, t10645, t10647, t10651, t10655, t14546, t14547, t14558, t14564, t2646, t2724, t2754, t4494, t4504, t4514, t4526, t820);
        let (t14577, t14581, t14586, t14587, t14590, t14593) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1498(t233, t4469, t869, t689, t2435, t4519, t1558, t2723, t836, t10529, t2782, t72);
        let (t14596, t14603, t14605) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1499(t14593, t686, t874, t1558, t2811, t2482, t122, t2723, t72, t676, t836, t879);
    (t14507, t14540, t14547, t14572, t14577, t14581, t14586, t14587, t14590, t14596, t14603, t14605)
}
