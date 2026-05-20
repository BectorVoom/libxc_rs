//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta911 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2927;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2930;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta911<F: Float>(t141: F, t2908: F, t77588: F, t77592: F, t77525: F, t77529: F, t63533: F, t63538: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t23475: F, t698: F, t41441: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t77681: F, t77705: F, t77732: F, t77747: F, t77801: F, t77824: F, t11404: F, t15343: F, t19156: F, t19167: F, t23706: F, t23717: F, t23723: F, t41756: F, t41779: F, t4685: F, t4708: F, t52809: F, t52820: F, t6158: F, t6177: F, t6206: F, t77639: F, t77641: F, t77643: F, t77645: F, t77647: F, t77657: F, t965: F, t973: F, t52546: F, t52547: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F, t77679: F, t41672: F, t77499: F, t77503: F, t77505: F, t77683: F, t77686: F, t77688: F, t77690: F, t77692: F, t77695: F, t77698: F, t77700: F, t41690: F, t63276: F, t63278: F, t77507: F, t77509: F, t77712: F, t77715: F, t77718: F, t77721: F, t77724: F, t77727: F, t77730: F, t41361: F, t51978: F, t52573: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77736: F, t77739: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77829, t77832, t77835, t77838, t77846) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2926::<F>(t141, t2908, t77588, t77592, t77525, t77529, t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551);
        let (t77858, t77860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2927::<F>(t23475, t698, t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t77863, t77873) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928::<F>(t77681, t77705, t77732, t77747, t77801, t77824, t77846, t77860, t11404, t15343, t19156, t19167, t23706, t23717, t23723, t41756, t41779, t4685, t4708, t52809, t52820, t6158, t6177, t6206, t77639, t77641, t77643, t77645, t77647, t77657, t965, t973);
        let (t77886, t77898) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929::<F>(t52546, t52547, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676, t77679, t41672, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700);
        let t77911 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2930::<F>(t41690, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727, t77730);
        let t77923 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931::<F>(t41361, t51978, t52573, t63320, t77515, t77518, t77521, t77527, t77531, t77535, t77736, t77739);
    (t77829, t77832, t77835, t77838, t77858, t77863, t77873, t77886, t77898, t77911, t77923)
}
