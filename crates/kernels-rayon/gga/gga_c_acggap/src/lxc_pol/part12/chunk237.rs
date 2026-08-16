//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 237/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk237(t43: f64, t50: f64, t316: f64, t880: f64, t243: f64, t75: f64, t288: f64, t98: f64, t47: f64, t818: f64, t824: f64, t100: f64, t52: f64, t830: f64, t833: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t882 = 0.65854491829355115987e0_f64 * t316 * t880;
    let t883 = t243 * t75;
    let t884 = t883 * t288;
    let t885 = 0.11696447245269292414e1_f64 * t884;
    let t886 = 1.0_f64 / t98;
    let t892 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t886 * t818 + 4.0_f64 / 3.0_f64 * t47 * t824);
    let t893 = 1.0_f64 / t100;
    let t899 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t893 * t830 + 4.0_f64 / 3.0_f64 * t52 * t833);
    (t882, t883, t884, t885, t886, t892, t893, t899)
}
