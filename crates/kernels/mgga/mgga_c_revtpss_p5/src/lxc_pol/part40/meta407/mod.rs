//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta407<F: Float>(t31451: F, t508: F, t1911: F, t8320: F, t569: F, t1312: F, t13426: F, t18227: F, t2201: F, t2322: F, t27123: F, t31401: F, t31403: F, t31407: F, t4248: F, t4254: F, t5523: F, t651: F, t8307: F, t8325: F, t8327: F, t8407: F, t8413: F) -> (F, F, F, F) {
        let (t31452, t31456, t31459, t31461) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1486::<F>(t31451, t508, t1911, t8320, t569, t1312, t13426, t18227, t2201, t2322, t27123, t31401, t31403, t31407, t4248, t4254, t5523, t651, t8307, t8325, t8327, t8407, t8413);
    (t31452, t31456, t31459, t31461)
}
