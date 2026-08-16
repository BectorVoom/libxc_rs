//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1178;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta337(t11465: f64, t23451: f64, t3014: f64, t981: f64, t3011: f64, t973: f64, t1610: f64, t19056: f64, t4590: f64, t6142: f64, t15421: f64, t6145: f64, t1609: f64, t6109: f64, t2926: f64, t11299: f64, t11144: f64, t22688: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23453, t23455, t23457, t23459, t23461, t23463, t23465) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1178(t11465, t23451, t3014, t981, t3011, t973, t1610, t19056, t4590, t6142, t15421, t6145);
        let (t23466, t23467, t23469, t23470) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1179(t1609, t6109, t2926, t11299, t11144, t22688);
    (t23453, t23455, t23457, t23459, t23461, t23463, t23465, t23466, t23467, t23469, t23470)
}
