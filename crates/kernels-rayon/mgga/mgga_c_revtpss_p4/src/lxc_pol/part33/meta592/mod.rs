//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta592(t25894: f64, t94394: f64, t25945: f64, t9285: f64, t25944: f64, t2482: f64, t7262: f64, t814: f64, t820: f64, t844: f64, t596: f64, t7269: f64, t3981: f64, t25981: f64, t843: f64, t2681: f64, t1401: f64, t533: f64, t816: f64, t92993: f64, t7259: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94395, t94407, t94409, t94423, t94429, t94443) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007(t25894, t94394, t25945, t9285, t25944, t2482, t7262, t814, t820, t844, t596, t7269);
        let (t94444, t94455, t94459, t94460, t94472, t94473) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2008(t3981, t94443, t25981, t820, t843, t2681, t7262, t1401, t533, t816, t92993, t7259, t9709);
    (t94395, t94407, t94409, t94423, t94429, t94443, t94444, t94455, t94459, t94460, t94472, t94473)
}
