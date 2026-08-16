//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta407<F: Float>(t1911: F, t8273: F, t1843: F, t1310: F, t8362: F, t31292: F, t508: F, t2178: F, t5787: F, t1312: F, t13426: F, t18227: F, t2179: F, t2181: F, t2322: F, t27123: F, t27126: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t8254: F, t8278: F, t8363: F, t8369: F) -> (F, F, F, F, F, F) {
        let (t31309, t31314, t31318, t31320, t31324, t31326) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1483::<F>(t1911, t8273, t1843, t1310, t8362, t31292, t508, t2178, t5787, t1312, t13426, t18227, t2179, t2181, t2322, t27123, t27126, t4248, t4254, t5523, t651, t7732, t8254, t8278, t8363, t8369);
    (t31309, t31314, t31318, t31320, t31324, t31326)
}
