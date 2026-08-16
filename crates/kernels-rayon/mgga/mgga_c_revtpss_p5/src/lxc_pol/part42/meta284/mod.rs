//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1040;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta284(t10671: f64, t813: f64, t2689: f64, t2694: f64, t243: f64, t247: f64, t9949: f64, t237: f64, t236: f64, t9646: f64, t9721: f64, t268: f64, t207: f64, t242: f64, t240: f64, t72: f64, t136: f64, t2476: f64, t2482: f64, t596: f64, t849: f64, t2677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10673, t10678, t10687, t10688, t10690) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1040(t10671, t813, t2689, t2694, t243, t247, t9949, t237, t236, t9646, t9721, t268);
        let (t10692, t10698, t10703, t10716, t10717) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1041(t10688, t10690, t207, t242, t240, t72, t136, t2476, t2482, t596, t849, t2677);
    (t10673, t10678, t10687, t10692, t10698, t10703, t10716, t10717)
}
