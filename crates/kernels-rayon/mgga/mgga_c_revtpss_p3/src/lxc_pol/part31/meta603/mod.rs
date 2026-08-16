//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2037;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta603(t27873: f64, t94886: f64, t27845: f64, t689: f64, t25904: f64, t25899: f64, t94649: f64, t97685: f64, t25898: f64, t7925: f64, t94849: f64, t1032: f64, t5710: f64, t1426: f64, t7063: f64, t7286: f64, t27852: f64, t25950: f64, t27888: f64, t25953: f64, t27884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97945, t97949, t97951, t97953, t97956, t97960) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2037(t27873, t94886, t27845, t689, t25904, t25899, t94649, t97685, t25898, t7925, t94849, t1032, t5710);
        let (t97961, t97964, t97968, t97974, t97976, t97985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2038(t1426, t97960, t7063, t7286, t27852, t689, t25904, t25899, t25950, t27888, t25953, t27884);
    (t97945, t97949, t97951, t97953, t97956, t97960, t97961, t97964, t97968, t97974, t97976, t97985)
}
