//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1711;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta460(t26265: f64, t3917: f64, t25899: f64, t26231: f64, t72: f64, t7531: f64, t686: f64, t7284: f64, t7289: f64, t136: f64, t2102: f64, t2457: f64, t25944: f64, t25950: f64, t7515: f64, t213: f64, t7506: f64, t1445: f64, t2103: f64, t25909: f64, t26232: f64, t26235: f64, t26238: f64, t26241: f64, t26246: f64, t26251: f64, t26253: f64, t26257: f64, t26263: f64, t4132: f64, t7292: f64, t7295: f64, t7511: f64, t7532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26266, t26268, t26270, t26271) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1710(t26265, t3917, t25899, t26231, t72, t7531, t686);
        let (t26272, t26274, t26276, t26277) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1711(t26271, t7284, t7289, t136, t2102, t2457);
        let (t26279, t26280, t26282, t26291) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1712(t25944, t26277, t25950, t7515, t213, t7506, t1445, t2103, t25909, t26232, t26235, t26238, t26241, t26246, t26251, t26253, t26257, t26263, t26266, t26268, t26272, t26274, t4132, t7292, t7295, t7511, t7532);
    (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277, t26279, t26280, t26282, t26291)
}
