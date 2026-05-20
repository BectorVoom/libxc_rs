//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta403<F: Float>(t31157: F, t569: F, t1453: F, t8320: F, t2198: F, t4151: F, t3813: F, t508: F, t1310: F, t10416: F, t1312: F, t13435: F, t13440: F, t18163: F, t2199: F, t2201: F, t2322: F, t4254: F, t5523: F, t651: F, t8307: F, t8321: F, t8325: F, t8327: F) -> (F, F, F, F, F, F, F) {
        let (t31158, t31161, t31164, t31169, t31172, t31201, t31204) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1479::<F>(t31157, t569, t1453, t8320, t2198, t4151, t3813, t508, t1310, t10416, t1312, t13435, t13440, t18163, t2199, t2201, t2322, t4254, t5523, t651, t8307, t8321, t8325, t8327);
    (t31158, t31161, t31164, t31169, t31172, t31201, t31204)
}
