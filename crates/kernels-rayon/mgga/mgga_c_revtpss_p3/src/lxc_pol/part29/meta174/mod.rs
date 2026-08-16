//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta174(t1280: f64, t3568: f64, t1284: f64, t487: f64, t1209: f64, t1287: f64, t3721: f64, t1269: f64, t473: f64, t1214: f64, t3584: f64, t3140: f64, t3596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk833(t1280, t3568, t1284, t487, t1209, t1287, t3721, t1269, t473, t1214, t3584, t3140, t3596);
    (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766)
}
