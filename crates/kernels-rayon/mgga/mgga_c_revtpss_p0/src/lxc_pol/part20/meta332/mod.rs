//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta332(t13126: f64, t460: f64, t13043: f64, t487: f64, t12051: f64, t471: f64, t3727: f64, t473: f64, t1214: f64, t11239: f64, t3596: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13127, t13128, t13129, t13130, t13133, t13134, t13141, t13142, t13143) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1250(t13126, t460, t13043, t487, t12051, t471, t3727, t473, t1214, t11239, t3596, t3603);
    (t13127, t13128, t13129, t13130, t13133, t13134, t13141, t13142, t13143)
}
