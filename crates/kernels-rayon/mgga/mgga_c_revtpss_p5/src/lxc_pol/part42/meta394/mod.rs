//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1331;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1332;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta394(t1134: f64, t20356: f64, t5071: f64, t5079: f64, t3390: f64, t6449: f64, t12331: f64, t6442: f64, t5087: f64, t3407: f64, t1139: f64, t20337: f64, t12254: f64, t20293: f64, t141: f64, t12542: f64, t12543: f64, t16710: f64, t16931: f64, t17131: f64, t17140: f64, t12261: f64, t12297: f64, t16706: f64, t16876: f64, t17115: f64, t17117: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20322: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20357, t20359, t20362, t20366, t20368, t20371, t20373) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1331(t1134, t20356, t5071, t5079, t3390, t6449, t12331, t6442, t5087, t3407, t1139, t20337);
        let (t20378, t20380) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1332(t12254, t20293, t141, t12542, t12543, t16710, t16931, t17131, t17140, t20366, t20368, t20371, t20373);
        let t20382 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1333(t12261, t12297, t16706, t16876, t17115, t17117, t20268, t20274, t20276, t20278, t20280, t20322, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20380);
    (t20357, t20359, t20362, t20366, t20368, t20371, t20373, t20378, t20382)
}
