//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 645/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk645(t29: f64, t32: f64, t8991: f64, t120: f64, t126: f64, t1631: f64, t2014: f64, t2016: f64, t2021: f64, t534: f64, t7914: f64, t7977: f64, t8691: f64, t8693: f64, t8696: f64, t8942: f64, t8948: f64, t8950: f64, t8956: f64, t8959: f64, t8963: f64, t8967: f64, t8972: f64, t8977: f64, t8978: f64, t8981: f64) -> (f64, f64) {
    let t8994 = t8991 / t32 / t29;
    let t8997 = 0.17557713923258613e0_f64 * t8691 * t120 - 0.35115427846517226e0_f64 * t8693 * t8942 + 0.33205381699090447729e-3_f64 * t8948 * t8950 + 0.23410285231011484e0_f64 * t8696 * t120 - 0.79692916077817074549e-2_f64 * t2014 * t8956 - 0.8854768453090786061e-3_f64 * t8959 * t2016 - 0.8854768453090786061e-3_f64 * t8963 * t8967 + 0.72343824494974941953e-3_f64 * t2014 * t8972 - 0.5116527820486904976e-1_f64 * t8977 * t8978 + 0.959348966341294683e-1_f64 * t2021 * t8981 - 0.25159457085530922489e-1_f64 * t7914 * t8978 - 0.532971647967385935e-1_f64 * t534 * t7977 * t126 + 0.41932428475884870816e-1_f64 * t1631 * t8981 - 0.91641760171536135284e-3_f64 * t8994 * t8978;
    (t8994, t8997)
}
