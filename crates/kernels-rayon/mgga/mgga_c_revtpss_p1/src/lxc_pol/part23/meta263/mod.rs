//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1467;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta263(t2482: f64, t27: f64, t4000: f64, t1419: f64, t4086: f64, t786: f64, t555: f64, t5744: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t2777: f64, t4092: f64, t2439: f64, t3999: f64, t123: f64, t212: f64, t2434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10001, t10013, t10014, t10022) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1467(t2482, t27, t4000, t1419, t4086, t786, t555, t5744);
        let (t10023, t10032, t10035, t10043, t10044, t10049, t10069) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1468(t10022, t786, t2435, t4093, t4083, t9303, t2777, t4092, t2439, t1419, t3999, t123, t212, t2434);
    (t10001, t10013, t10014, t10022, t10023, t10032, t10035, t10043, t10044, t10049, t10069)
}
