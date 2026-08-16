//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1896;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta525<F: Float>(t14230: F, t27980: F, t1445: F, t213: F, t25930: F, t25955: F, t26040: F, t26043: F, t26051: F, t26055: F, t26058: F, t27837: F, t27868: F, t27909: F, t27961: F, t27966: F, t27969: F, t27973: F, t561: F, t5775: F, t7279: F, t7298: F, t212: F, t7910: F, t1358: F, t689: F, t7925: F) -> (F, F, F, F, F, F) {
        let (t27981, t27984) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1896::<F>(t14230, t27980, t1445, t213, t25930, t25955, t26040, t26043, t26051, t26055, t26058, t27837, t27868, t27909, t27961, t27966, t27969, t27973, t561, t5775, t7279, t7298);
        let (t27985, t27986, t27987, t27989) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1897::<F>(t212, t7910, t1358, t689, t7925);
    (t27981, t27984, t27985, t27986, t27987, t27989)
}
