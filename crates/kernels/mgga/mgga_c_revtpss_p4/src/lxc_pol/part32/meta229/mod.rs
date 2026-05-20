//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk983;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta229<F: Float>(t6141: F, t935: F, t915: F, t2926: F, t6109: F, t2924: F, t2930: F, t4571: F, t6094: F, t6098: F, t6102: F, t1621: F, t954: F, t2950: F, t2957: F, t4620: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F) -> (F, F, F, F, F, F, F, F) {
        let (t6142, t6144, t6145, t6147, t6152, t6157) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk983::<F>(t6141, t935, t915, t2926, t6109, t2924, t2930, t4571, t6094, t6098, t6102, t1621);
        let (t6158, t6173) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk984::<F>(t6157, t954, t2950, t2957, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
    (t6142, t6144, t6145, t6147, t6152, t6157, t6158, t6173)
}
