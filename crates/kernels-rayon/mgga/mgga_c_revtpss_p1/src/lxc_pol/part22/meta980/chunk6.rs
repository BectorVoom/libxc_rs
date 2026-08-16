//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3308/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3308(t2782: f64, t2797: f64, t62695: f64, t39649: f64, t39652: f64, t39662: f64, t39673: f64, t39678: f64, t39683: f64, t51390: f64, t51403: f64, t51408: f64, t61648: f64, t62682: f64, t62684: f64, t62693: f64, t820: f64, t879: f64) -> f64 {
    let t62697 = t2782 * t2797 * t62695;
    let t62705 = -0.19514881078765566038e-1_f64 * t62682 + t39649 - t39652 - 0.13009920719177044025e-2_f64 * t62684 + 0.520396828767081761e-2_f64 * t51390 - 0.2601984143835408805e-1_f64 * t39662 + 0.92526556154787137112e-2_f64 * t39673 - 0.11565819519348392139e-2_f64 * t39678 + 0.10975748638225852664e-1_f64 * t62693 + 0.10975748638225852664e-1_f64 * t62697 - 0.34146773541147097178e-1_f64 * t51403 + 0.23131639038696784278e-2_f64 * t39683 - 0.60712963356159538786e-1_f64 * t51408 - 0.13170898365871023197e1_f64 * t820 * t879 * t61648;
    t62705
}
