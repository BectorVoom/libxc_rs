//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta628(t15731: f64, t7122: f64, t25512: f64, t4820: f64, t25515: f64, t370: f64, t16087: f64, t16055: f64, t27493: f64, t15925: f64, t25516: f64, t1087: f64, t93751: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100002, t100006, t100007, t100008, t100024, t100025, t100030) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2082(t15731, t7122, t25512, t4820, t25515, t370, t16087, t16055, t27493, t15925, t25516, t1087, t93751);
    (t100002, t100006, t100007, t100008, t100024, t100025, t100030)
}
