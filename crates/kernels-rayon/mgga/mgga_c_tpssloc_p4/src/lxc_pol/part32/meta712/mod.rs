//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2233;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2235;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2236;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2238;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2239;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2240;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta712(t17037: f64, t1888: f64, t22996: f64, t232: f64, t58204: f64, t6646: f64, t2632: f64, t58166: f64, t28423: f64, t6579: f64, t28427: f64, t1902: f64, t5611: f64, t16830: f64, t25255: f64, t25262: f64, t2617: f64, t28413: f64, t4234: f64, t4291: f64, t5585: f64, t812: f64, t81679: f64, t829: f64, t87154: f64, t92516: f64, t98461: f64, t98464: f64, t98467: f64, t98471: f64, t98475: f64, t25038: f64, t25248: f64, t25249: f64, t4119: f64, t28419: f64, t23035: f64, t23153: f64, t5527: f64, t6637: f64, t22893: f64, t28341: f64, t81640: f64, t1484: f64, t6552: f64, t87586: f64, t1509: f64, t7510: f64, t16815: f64, t22986: f64, t2647: f64, t22992: f64, t25269: f64, t25297: f64, t4166: f64, t4182: f64, t4281: f64, t5612: f64, t81615: f64, t87166: f64, t87521: f64, t87523: f64, t87534: f64, t92543: f64, t5584: f64, t58226: f64, t23110: f64, t23185: f64, t28418: f64, t59331: f64, t23168: f64, t28330: f64, t13397: f64, t16816: f64, t25261: f64, t81633: f64, t87536: f64, t87545: f64, t87547: f64, t87566: f64, t87582: f64, t87584: f64, t87602: f64, t5631: f64, t828: f64, t25319: f64, t16935: f64, t17034: f64, t25281: f64, t4162: f64, t5575: f64, t6660: f64, t7535: f64, t81689: f64, t81717: f64, t82011: f64, t87604: f64, t87613: f64, t87619: f64, t87635: f64, t87669: f64, t87680: f64, t92781: f64, t92794: f64, t28406: f64, t814: f64, t234: f64, t776: f64, t16758: f64, t5593: f64, t81865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98478, t98482, t98486, t98488, t98490, t98494) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2233(t17037, t1888, t22996, t232, t58204, t6646, t2632, t58166, t28423, t6579, t28427, t1902, t5611);
        let t98497 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234(t16830, t25255, t25262, t2617, t28413, t4234, t4291, t5585, t812, t81679, t829, t87154, t92516, t98461, t98464, t98467, t98471, t98475, t98478, t98482, t98486, t98488, t98490, t98494);
        let (t98502, t98505, t98513, t98516) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2235(t25038, t25248, t25249, t4119, t28419, t6579, t23035, t23153, t5527, t6637, t22893, t28341, t81640);
        let (t98520, t98524, t98530, t98534) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2236(t1484, t6552, t6637, t87586, t1509, t7510, t1888, t232, t58166, t6646, t16815, t22986, t2647);
        let t98536 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237(t22992, t25269, t25297, t4166, t4182, t4281, t5612, t812, t81615, t87166, t87521, t87523, t87534, t92543, t98502, t98505, t98513, t98516, t98520, t98524, t98530, t98534);
        let (t98541, t98546, t98549, t98553, t98564) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2238(t1902, t5584, t1888, t232, t58226, t6646, t23110, t23185, t28418, t59331, t23168, t28330);
        let t98566 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2239(t13397, t16816, t25261, t4182, t4234, t4281, t4291, t81633, t829, t87536, t87545, t87547, t87566, t87582, t87584, t87602, t98494, t98541, t98546, t98549, t98553, t98564);
        let t98587 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2240(t1888, t232, t5631, t6646, t828, t25319, t4119, t6552, t6637, t16935, t17034, t25261, t25281, t4162, t4281, t5575, t6660, t7535, t81689, t81717, t82011, t87604, t87613, t87619, t87635, t87669, t87680, t92781, t92794);
        let (t98592, t98601, t98608, t98610) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2241(t28406, t814, t234, t5631, t6552, t6637, t776, t16758, t22986, t2647, t6646, t5593, t81865);
    (t98497, t98524, t98536, t98566, t98587, t98592, t98601, t98608, t98610)
}
