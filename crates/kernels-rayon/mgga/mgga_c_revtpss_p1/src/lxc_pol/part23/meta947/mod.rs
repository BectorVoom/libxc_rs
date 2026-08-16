//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta947 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3131;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta947(t5245: f64, t6628: f64, t20816: f64, t5293: f64, t12855: f64, t17505: f64, t17729: f64, t20317: f64, t21037: f64, t21184: f64, t21242: f64, t24836: f64, t3367: f64, t3604: f64, t3626: f64, t3720: f64, t4181: f64, t44484: f64, t44500: f64, t44502: f64, t45371: f64, t5270: f64, t5297: f64, t5348: f64, t5352: f64, t6587: f64, t70303: f64, t70800: f64, t82293: f64, t24611: f64, t3172: f64, t3711: f64, t1042: f64, t1261: f64, t17202: f64, t17344: f64, t1789: f64, t20703: f64, t20982: f64, t21095: f64, t21203: f64, t5299: f64, t5381: f64, t56254: f64, t69668: f64, t69674: f64, t69698: f64, t69700: f64, t69795: f64, t78785: f64, t78790: f64, t5819: f64, t81128: f64, t81130: f64, t81132: f64, t81134: f64, t81136: f64, t81138: f64, t81145: f64, t81148: f64, t81150: f64, t81152: f64, t81254: f64, t81257: f64, t81259: f64, t81261: f64, t81264: f64, t81266: f64, t81307: f64, t81309: f64, t81313: f64, t81315: f64, t81317: f64, t81322: f64, t81326: f64, t81328: f64, t81330: f64, t81333: f64, t81336: f64, t81338: f64, t81341: f64, t81343: f64, t81352: f64, t81558: f64, t81560: f64, t81562: f64, t81566: f64, t81570: f64, t81573: f64, t81575: f64, t81577: f64, t81580: f64, t81582: f64, t81589: f64, t81591: f64, t81593: f64, t81596: f64, t81599: f64, t81601: f64, t81604: f64, t81606: f64, t81609: f64, t81612: f64, t81614: f64, t81618: f64, t81621: f64, t81623: f64, t81625: f64, t81627: f64, t81629: f64, t81631: f64, t81633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t82321, t82340) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128(t5245, t6628, t20816, t5293, t12855, t17505, t17729, t20317, t21037, t21184, t21242, t24836, t3367, t3604, t3626, t3720, t4181, t44484, t44500, t44502, t45371, t5270, t5297, t5348, t5352, t6587, t70303, t70800, t82293);
        let t82367 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129(t24611, t3172, t3711, t1042, t1261, t17202, t17344, t1789, t20703, t20982, t21095, t21203, t5299, t5381, t56254, t69668, t69674, t69698, t69700, t69795, t78785, t78790);
        let (t82368, t82385) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130(t5245, t5819, t81128, t81130, t81132, t81134, t81136, t81138, t81145, t81148, t81150, t81152, t81254, t81257, t81259, t81261, t81264, t81266, t81307, t81309, t81313, t81315);
        let t82386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3131(t81317, t81322, t81326, t81328, t81330, t81333, t81336, t81338, t81341, t81343, t81352, t81558, t81560, t81562, t81566, t81570, t81573, t81575, t81577, t81580);
        let t82388 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3132(t81582, t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612, t81614, t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633);
    (t82321, t82340, t82367, t82368, t82385, t82386, t82388)
}
