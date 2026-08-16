//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta539(t1248: f64, t8201: f64, t1287: f64, t8197: f64, t1209: f64, t8190: f64, t1294: f64, t7652: f64, t1770: f64, t2142: f64, t1214: f64, t7637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29212, t29213, t29216, t29217, t29220, t29224, t29227, t29233) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1966(t1248, t8201, t1287, t8197, t1209, t8190, t1294, t7652, t1770, t2142, t1214, t7637);
    (t29212, t29213, t29216, t29217, t29220, t29224, t29227, t29233)
}
