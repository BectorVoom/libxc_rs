//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta311(t3930: f64, t6846: f64, t221: f64, t4019: f64, t6862: f64, t10001: f64, t6800: f64, t72: f64, t757: f64, t1317: f64, t6801: f64, t1320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22179, t22182, t22183, t22185, t22186, t22188, t22191) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1098(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t1320);
    (t22179, t22182, t22183, t22185, t22186, t22188, t22191)
}
