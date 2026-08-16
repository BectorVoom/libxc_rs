//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1192;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta293(t1419: f64, t4086: f64, t786: f64, t555: f64, t5744: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t2777: f64, t4092: f64, t2439: f64, t3999: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10014, t10022, t10023, t10032, t10035, t10044) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1192(t1419, t4086, t786, t555, t5744, t2435, t4093, t4083, t9303, t2777, t4092, t2439);
        let (t10049, t10069, t10070, t10073) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1193(t1419, t3999, t123, t212, t2434, t4089, t138, t2438, t785);
    (t10014, t10022, t10023, t10032, t10035, t10044, t10049, t10069, t10070, t10073)
}
