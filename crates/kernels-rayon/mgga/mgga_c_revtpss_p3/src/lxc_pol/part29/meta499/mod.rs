//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta499(t28799: f64, t28822: f64, t28861: f64, t28923: f64, t532: f64, t1450: f64, t5627: f64, t9069: f64, t26411: f64, t7900: f64, t28176: f64, t7488: f64, t531: f64, t8107: f64, t7238: f64, t2014: f64, t2056: f64, t2093: f64, t2108: f64, t27123: f64, t27126: f64, t27833: f64, t28167: f64, t28760: f64, t4248: f64, t5787: f64, t651: f64, t7235: f64, t7367: f64, t7374: f64, t7489: f64, t7732: f64, t7898: f64, t8079: f64, t8109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28925, t28926, t28927, t28929, t28932, t28935) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1814(t28799, t28822, t28861, t28923, t532, t1450, t5627, t9069, t26411, t7900, t28176, t7488);
        let (t28938, t28939, t28942) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1815(t531, t8107, t7238, t2014, t2056, t2093, t2108, t27123, t27126, t27833, t28167, t28760, t28927, t28929, t28932, t28935, t4248, t5787, t651, t7235, t7367, t7374, t7489, t7732, t7898, t8079, t8109);
    (t28925, t28926, t28927, t28929, t28932, t28935, t28938, t28939, t28942)
}
