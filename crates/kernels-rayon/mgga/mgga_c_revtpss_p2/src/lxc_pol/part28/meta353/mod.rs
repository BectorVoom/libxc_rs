//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta353(t1063: f64, t11988: f64, t1062: f64, t3196: f64, t3223: f64, t3229: f64, t369: f64, t361: f64, t351: f64, t3106: f64, t3111: f64, t3156: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11989, t11991, t11994, t12002, t12004, t12007, t12009) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1373(t1063, t11988, t1062, t3196, t3223, t3229, t369, t361, t351, t3106, t3111, t3156, t3172);
    (t11989, t11991, t11994, t12002, t12004, t12007, t12009)
}
