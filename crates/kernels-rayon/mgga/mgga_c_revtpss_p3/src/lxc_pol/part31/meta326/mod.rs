//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta326(t1025: f64, t11817: f64, t271: f64, t2857: f64, t283: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t1086: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11818, t11821, t11852, t11858, t11859, t11860, t11865) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1329(t1025, t11817, t271, t2857, t283, t3298, t994, t4891, t3154, t999, t1086, t3046);
    (t11818, t11821, t11852, t11858, t11859, t11860, t11865)
}
