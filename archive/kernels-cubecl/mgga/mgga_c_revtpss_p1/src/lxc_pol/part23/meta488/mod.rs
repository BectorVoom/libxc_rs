//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1958;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1959;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta488<F: Float>(t1134: F, t20356: F, t5071: F, t5079: F, t3390: F, t6449: F, t12331: F, t6442: F, t5087: F, t3407: F, t1139: F, t20337: F, t12254: F, t20293: F, t141: F, t12542: F, t12543: F, t16710: F, t16931: F, t17131: F, t17140: F, t12261: F, t12297: F, t16706: F, t16876: F, t17115: F, t17117: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20322: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20357, t20359, t20362, t20365, t20366, t20368, t20371, t20373) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1958::<F>(t1134, t20356, t5071, t5079, t3390, t6449, t12331, t6442, t5087, t3407, t1139, t20337);
        let (t20377, t20378, t20380) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1959::<F>(t12254, t20293, t141, t12542, t12543, t16710, t16931, t17131, t17140, t20366, t20368, t20371, t20373);
        let t20382 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1960::<F>(t12261, t12297, t16706, t16876, t17115, t17117, t20268, t20274, t20276, t20278, t20280, t20322, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20380);
    (t20357, t20359, t20362, t20365, t20366, t20368, t20371, t20373, t20377, t20378, t20382)
}
