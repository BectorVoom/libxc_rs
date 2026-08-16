//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta358(t12079: f64, t24078: f64, t1668: f64, t3302: f64, t357: f64, t19572: f64, t4982: f64, t6299: f64, t4893: f64, t12168: f64, t1651: f64, t19556: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t24079, t24083, t24084, t24090, t24093, t24098) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1226(t12079, t24078, t1668, t3302, t357, t19572, t4982, t6299, t4893, t12168, t1651, t19556);
    (t24079, t24083, t24084, t24090, t24093, t24098)
}
