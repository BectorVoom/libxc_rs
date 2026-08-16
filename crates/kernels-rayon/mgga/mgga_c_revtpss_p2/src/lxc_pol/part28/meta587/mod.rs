//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2054;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta587(t26009: f64, t9802: f64, t26004: f64, t3961: f64, t64: f64, t9990: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25986: f64, t2661: f64, t9980: f64, t26024: f64, t3926: f64, t4059: f64, t25981: f64, t27: f64, t10003: f64, t25997: f64, t9970: f64, t550: f64, t7021: f64, t3946: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94484, t94485, t94491, t94497, t94498, t94501) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2054(t26009, t9802, t26004, t3961, t64, t9990, t2482, t596, t7262, t4021, t25986, t2661, t9980);
        let (t94503, t94505, t94509, t94511, t94514) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2055(t26024, t3926, t4059, t2482, t25981, t27, t10003, t25997, t9970, t550, t7021, t3946);
    (t94484, t94485, t94491, t94497, t94498, t94501, t94503, t94505, t94509, t94511, t94514)
}
