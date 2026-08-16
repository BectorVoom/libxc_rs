//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2030;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta570(t93174: f64, t93371: f64, t25410: f64, t93341: f64, t25413: f64, t25374: f64, t93169: f64, t93191: f64, t2439: f64, t7048: f64, t780: f64, t785: f64, t25310: f64, t25331: f64, t25412: f64, t93329: f64, t25411: f64, t25431: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t25624: f64, t3056: f64, t7143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93372, t93374, t93375, t93377, t93378, t93382) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2030(t93174, t93371, t25410, t93341, t25413, t25374, t93169, t93191, t2439, t7048, t780, t785);
        let (t93384, t93387, t93389, t93391, t93404, t93429) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2031(t25310, t25331, t25412, t93329, t25411, t25431, t2435, t25339, t11064, t7086, t25624, t3056, t7143);
    (t93372, t93374, t93375, t93377, t93378, t93382, t93384, t93387, t93389, t93391, t93404, t93429)
}
