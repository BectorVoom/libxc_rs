//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta578(t94471: f64, t7259: f64, t9709: f64, t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64, t26009: f64, t9802: f64, t26004: f64, t3961: f64, t64: f64, t9990: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25986: f64, t2661: f64, t9980: f64, t26024: f64, t3926: f64, t4059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94472, t94474, t94477, t94479, t94484, t94485) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2029(t94471, t7259, t9709, t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802, t26004, t3961);
        let (t94491, t94497, t94498, t94501, t94503, t94505) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2030(t64, t9990, t2482, t596, t7262, t4021, t25986, t2661, t9980, t26024, t3926, t4059);
    (t94472, t94474, t94477, t94479, t94484, t94485, t94491, t94497, t94498, t94501, t94503, t94505)
}
