//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta276(t283: f64, t2857: f64, t66: f64, t11145: f64, t247: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t11659: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11852, t11853, t11855, t11858, t11859, t11860, t11861, t11862) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1132(t283, t2857, t66, t11145, t247, t3298, t994, t4891, t3154, t999, t11659, t3117);
    (t11852, t11853, t11855, t11858, t11859, t11860, t11861, t11862)
}
