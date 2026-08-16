//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1896;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta525(t14230: f64, t27980: f64, t1445: f64, t213: f64, t25930: f64, t25955: f64, t26040: f64, t26043: f64, t26051: f64, t26055: f64, t26058: f64, t27837: f64, t27868: f64, t27909: f64, t27961: f64, t27966: f64, t27969: f64, t27973: f64, t561: f64, t5775: f64, t7279: f64, t7298: f64, t212: f64, t7910: f64, t1358: f64, t689: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t27981, t27984) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1896(t14230, t27980, t1445, t213, t25930, t25955, t26040, t26043, t26051, t26055, t26058, t27837, t27868, t27909, t27961, t27966, t27969, t27973, t561, t5775, t7279, t7298);
        let (t27985, t27986, t27987, t27989) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1897(t212, t7910, t1358, t689, t7925);
    (t27981, t27984, t27985, t27986, t27987, t27989)
}
