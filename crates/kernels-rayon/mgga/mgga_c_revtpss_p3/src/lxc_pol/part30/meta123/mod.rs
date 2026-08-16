//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk697;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk698;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta123(t2724: f64, t827: f64, t828: f64, t159: f64, t243: f64, t216: f64, t124: f64, t2394: f64, t800: f64, t2712: f64, t785: f64, t225: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64, t839: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2726, t2729, t2730, t2732, t2735) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk697(t2724, t827, t828, t159, t243, t216, t124, t2394, t800, t2712, t785);
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk698(t225, t2735, t826, t849, t820, t823, t843);
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk699(t2741, t839, t241, t820, t823);
    (t2726, t2729, t2730, t2732, t2735, t2736, t2737, t2739, t2741, t2742, t2745)
}
