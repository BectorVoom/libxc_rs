//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 722/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk722(t1005: f64, t4466: f64, t126: f64, t20049: f64, t120: f64, t1631: f64, t16853: f64, t2014: f64, t2021: f64, t20589: f64, t20592: f64, t20596: f64, t20599: f64, t20603: f64, t20607: f64, t20612: f64, t20615: f64, t3359: f64, t534: f64, t7914: f64, t8948: f64, t8963: f64, t8977: f64, t8994: f64) -> f64 {
    let t20618 = t1005 * t4466;
    let t20623 = t20049 * t126;
    let t20630 = 0.17557713923258613e0_f64 * t20589 * t120 - 0.35115427846517226e0_f64 * t3359 * t20592 + 0.33205381699090447729e-3_f64 * t8948 * t20596 + 0.23410285231011484e0_f64 * t20599 * t120 - 0.79692916077817074549e-2_f64 * t2014 * t20603 - t16853 - 0.8854768453090786061e-3_f64 * t8963 * t20607 + 0.72343824494974941953e-3_f64 * t2014 * t20612 - 0.5116527820486904976e-1_f64 * t8977 * t20615 + 0.959348966341294683e-1_f64 * t2021 * t20618 - 0.25159457085530922489e-1_f64 * t7914 * t20615 - 0.532971647967385935e-1_f64 * t534 * t20623 + 0.41932428475884870816e-1_f64 * t1631 * t20618 - 0.91641760171536135284e-3_f64 * t8994 * t20615;
    t20630
}
