//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1328;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta373(t16088: f64, t16094: f64, t3169: f64, t4820: f64, t3188: f64, t4817: f64, t1065: f64, t4772: f64, t247: f64, t3109: f64, t4583: f64, t1063: f64, t3172: f64, t4868: f64, t1041: f64, t3168: f64, t4878: f64, t11150: f64, t3181: f64, t11144: f64, t11852: f64, t3124: f64, t1655: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16095, t16121, t16134, t16138, t16160) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1328(t16088, t16094, t3169, t4820, t3188, t4817, t1065, t4772, t247, t3109, t4583, t1063);
        let (t16165, t16190, t16199, t16208, t16218, t16219) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1329(t3172, t4868, t1041, t3168, t4878, t11150, t3181, t11144, t11852, t3124, t4820, t1655, t697);
    (t16095, t16121, t16134, t16138, t16160, t16165, t16190, t16199, t16208, t16218, t16219)
}
