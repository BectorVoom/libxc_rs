//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta911 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2927;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2930;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta911(t141: f64, t2908: f64, t77588: f64, t77592: f64, t77525: f64, t77529: f64, t63533: f64, t63538: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t23475: f64, t698: f64, t41441: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t77681: f64, t77705: f64, t77732: f64, t77747: f64, t77801: f64, t77824: f64, t11404: f64, t15343: f64, t19156: f64, t19167: f64, t23706: f64, t23717: f64, t23723: f64, t41756: f64, t41779: f64, t4685: f64, t4708: f64, t52809: f64, t52820: f64, t6158: f64, t6177: f64, t6206: f64, t77639: f64, t77641: f64, t77643: f64, t77645: f64, t77647: f64, t77657: f64, t965: f64, t973: f64, t52546: f64, t52547: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64, t77679: f64, t41672: f64, t77499: f64, t77503: f64, t77505: f64, t77683: f64, t77686: f64, t77688: f64, t77690: f64, t77692: f64, t77695: f64, t77698: f64, t77700: f64, t41690: f64, t63276: f64, t63278: f64, t77507: f64, t77509: f64, t77712: f64, t77715: f64, t77718: f64, t77721: f64, t77724: f64, t77727: f64, t77730: f64, t41361: f64, t51978: f64, t52573: f64, t63320: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77736: f64, t77739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77829, t77832, t77835, t77838, t77846) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926(t141, t2908, t77588, t77592, t77525, t77529, t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551);
        let (t77858, t77860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2927(t23475, t698, t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t77863, t77873) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928(t77681, t77705, t77732, t77747, t77801, t77824, t77846, t77860, t11404, t15343, t19156, t19167, t23706, t23717, t23723, t41756, t41779, t4685, t4708, t52809, t52820, t6158, t6177, t6206, t77639, t77641, t77643, t77645, t77647, t77657, t965, t973);
        let (t77886, t77898) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929(t52546, t52547, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676, t77679, t41672, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700);
        let t77911 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2930(t41690, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727, t77730);
        let t77923 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931(t41361, t51978, t52573, t63320, t77515, t77518, t77521, t77527, t77531, t77535, t77736, t77739);
    (t77829, t77832, t77835, t77838, t77858, t77863, t77873, t77886, t77898, t77911, t77923)
}
