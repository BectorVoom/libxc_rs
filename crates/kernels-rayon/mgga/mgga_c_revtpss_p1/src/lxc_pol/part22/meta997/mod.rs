//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta997 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta997(t15393: f64, t15421: f64, t15397: f64, t52224: f64, t2918: f64, t2924: f64, t6110: f64, t11385: f64, t2875: f64, t6145: f64, t198: f64, t3336: f64, t336: f64, t63589: f64, t63592: f64, t63596: f64, t63600: f64, t63601: f64, t63607: f64, t63609: f64, t63612: f64, t63615: f64, t63618: f64, t15474: f64, t1610: f64, t2874: f64, t11299: f64, t11528: f64, t19327: f64, t19128: f64, t934: f64, t6142: f64, t19330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63620, t63622, t63625, t63628, t63629) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387(t15393, t15421, t15397, t52224, t2918, t2924, t6110, t11385, t2875, t6145, t198, t3336, t336, t63589, t63592, t63596, t63600, t63601, t63607, t63609, t63612, t63615, t63618);
        let (t63633, t63636, t63638, t63641, t63644, t63647) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388(t15474, t1610, t2874, t11299, t2918, t6145, t11528, t19327, t19128, t934, t6142, t19330, t2875);
    (t63620, t63622, t63625, t63628, t63629, t63633, t63636, t63638, t63641, t63644, t63647)
}
