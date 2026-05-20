//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta947 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3131;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta947<F: Float>(t5245: F, t6628: F, t20816: F, t5293: F, t12855: F, t17505: F, t17729: F, t20317: F, t21037: F, t21184: F, t21242: F, t24836: F, t3367: F, t3604: F, t3626: F, t3720: F, t4181: F, t44484: F, t44500: F, t44502: F, t45371: F, t5270: F, t5297: F, t5348: F, t5352: F, t6587: F, t70303: F, t70800: F, t82293: F, t24611: F, t3172: F, t3711: F, t1042: F, t1261: F, t17202: F, t17344: F, t1789: F, t20703: F, t20982: F, t21095: F, t21203: F, t5299: F, t5381: F, t56254: F, t69668: F, t69674: F, t69698: F, t69700: F, t69795: F, t78785: F, t78790: F, t5819: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t81145: F, t81148: F, t81150: F, t81152: F, t81254: F, t81257: F, t81259: F, t81261: F, t81264: F, t81266: F, t81307: F, t81309: F, t81313: F, t81315: F, t81317: F, t81322: F, t81326: F, t81328: F, t81330: F, t81333: F, t81336: F, t81338: F, t81341: F, t81343: F, t81352: F, t81558: F, t81560: F, t81562: F, t81566: F, t81570: F, t81573: F, t81575: F, t81577: F, t81580: F, t81582: F, t81589: F, t81591: F, t81593: F, t81596: F, t81599: F, t81601: F, t81604: F, t81606: F, t81609: F, t81612: F, t81614: F, t81618: F, t81621: F, t81623: F, t81625: F, t81627: F, t81629: F, t81631: F, t81633: F) -> (F, F, F, F, F, F, F) {
        let (t82321, t82340) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3128::<F>(t5245, t6628, t20816, t5293, t12855, t17505, t17729, t20317, t21037, t21184, t21242, t24836, t3367, t3604, t3626, t3720, t4181, t44484, t44500, t44502, t45371, t5270, t5297, t5348, t5352, t6587, t70303, t70800, t82293);
        let t82367 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3129::<F>(t24611, t3172, t3711, t1042, t1261, t17202, t17344, t1789, t20703, t20982, t21095, t21203, t5299, t5381, t56254, t69668, t69674, t69698, t69700, t69795, t78785, t78790);
        let (t82368, t82385) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130::<F>(t5245, t5819, t81128, t81130, t81132, t81134, t81136, t81138, t81145, t81148, t81150, t81152, t81254, t81257, t81259, t81261, t81264, t81266, t81307, t81309, t81313, t81315);
        let t82386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3131::<F>(t81317, t81322, t81326, t81328, t81330, t81333, t81336, t81338, t81341, t81343, t81352, t81558, t81560, t81562, t81566, t81570, t81573, t81575, t81577, t81580);
        let t82388 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3132::<F>(t81582, t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612, t81614, t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633);
    (t82321, t82340, t82367, t82368, t82385, t82386, t82388)
}
