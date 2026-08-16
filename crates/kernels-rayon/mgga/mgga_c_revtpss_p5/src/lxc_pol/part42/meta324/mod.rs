//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1108;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1109;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta324(t2777: f64, t5759: f64, t2439: f64, t1398: f64, t1892: f64, t4086: f64, t543: f64, t2782: f64, t5659: f64, t72: f64, t686: f64, t4101: f64, t136: f64, t1883: f64, t2457: f64, t10139: f64, t13926: f64, t4100: f64, t10014: f64, t5741: f64, t13790: f64, t10022: f64, t786: f64, t4104: f64, t2470: f64, t5740: f64, t1432: f64, t5763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14203, t14209, t14218) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1108(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14221, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1109(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14233, t14239, t14241, t14243, t14252) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1110(t10022, t14230, t2782, t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763);
    (t14203, t14209, t14218, t14221, t14227, t14229, t14233, t14239, t14241, t14243, t14252)
}
