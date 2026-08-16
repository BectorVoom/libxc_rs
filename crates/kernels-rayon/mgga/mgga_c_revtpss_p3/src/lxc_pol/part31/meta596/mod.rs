//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2025;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta596(t2030: f64, t47567: f64, t26069: f64, t94806: f64, t26054: f64, t9686: f64, t25877: f64, t94801: f64, t1419: f64, t786: f64, t2023: f64, t4075: f64, t2453: f64, t25949: f64, t25946: f64, t25939: f64, t40270: f64, t10073: f64, t25920: f64, t25938: f64, t25898: f64, t10115: f64, t2024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94867, t94876, t94884, t94886, t94889, t94890, t94901) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2025(t2030, t47567, t26069, t94806, t26054, t9686, t25877, t94801, t1419, t786, t2023, t4075);
        let (t94914, t94917, t94919, t94921, t94931) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2026(t2453, t25949, t25946, t25939, t40270, t10073, t25920, t25938, t25898, t94889, t10115, t2024);
    (t94867, t94876, t94884, t94886, t94890, t94901, t94914, t94917, t94919, t94921, t94931)
}
