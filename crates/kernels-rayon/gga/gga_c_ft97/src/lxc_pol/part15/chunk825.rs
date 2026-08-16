//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 825/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk825(t21373: f64, t278: f64, t10339: f64, t10355: f64, t19167: f64, t2014: f64, t21821: f64, t21825: f64, t22096: f64, t22100: f64, t22107: f64, t22111: f64, t22116: f64, t22119: f64, t22122: f64, t2394: f64, t2710: f64, t274: f64, t4068: f64, t807: f64, t8948: f64, t8963: f64, t9609: f64) -> f64 {
    let t22127 = t21373 * t278;
    let t22134 = 0.17557713923258613e0_f64 * t21821 * t274 - 0.35115427846517226e0_f64 * t4068 * t22096 + 0.33205381699090447729e-3_f64 * t8948 * t22100 + 0.23410285231011484e0_f64 * t21825 * t274 - 0.79692916077817074549e-2_f64 * t2014 * t22107 - t19167 - 0.8854768453090786061e-3_f64 * t8963 * t22111 + 0.72343824494974941953e-3_f64 * t2014 * t22116 - 0.5116527820486904976e-1_f64 * t10339 * t22119 + 0.959348966341294683e-1_f64 * t2710 * t22122 - 0.25159457085530922489e-1_f64 * t9609 * t22119 - 0.532971647967385935e-1_f64 * t807 * t22127 + 0.41932428475884870816e-1_f64 * t2394 * t22122 - 0.91641760171536135284e-3_f64 * t10355 * t22119;
    t22134
}
