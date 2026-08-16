//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 764/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk764(t194: f64, t197: f64, t8991: f64, t10305: f64, t10308: f64, t10309: f64, t10313: f64, t10316: f64, t10321: f64, t10329: f64, t10334: f64, t10339: f64, t10340: f64, t10343: f64, t2014: f64, t2394: f64, t2705: f64, t2710: f64, t274: f64, t278: f64, t807: f64, t8948: f64, t8959: f64, t8963: f64, t9600: f64, t9609: f64) -> (f64, f64) {
    let t10355 = t8991 / t197 / t194;
    let t10358 = 0.17557713923258613e0_f64 * t10305 * t274 - 0.35115427846517226e0_f64 * t10308 * t10309 + 0.33205381699090447729e-3_f64 * t8948 * t10313 + 0.23410285231011484e0_f64 * t10316 * t274 - 0.79692916077817074549e-2_f64 * t2014 * t10321 - 0.8854768453090786061e-3_f64 * t8959 * t2705 - 0.8854768453090786061e-3_f64 * t8963 * t10329 + 0.72343824494974941953e-3_f64 * t2014 * t10334 - 0.5116527820486904976e-1_f64 * t10339 * t10340 + 0.959348966341294683e-1_f64 * t2710 * t10343 - 0.25159457085530922489e-1_f64 * t9609 * t10340 - 0.532971647967385935e-1_f64 * t807 * t9600 * t278 + 0.41932428475884870816e-1_f64 * t2394 * t10343 - 0.91641760171536135284e-3_f64 * t10355 * t10340;
    (t10355, t10358)
}
