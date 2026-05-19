//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 825/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk825<F: Float>(t21373: F, t278: F, t10339: F, t10355: F, t19167: F, t2014: F, t21821: F, t21825: F, t22096: F, t22100: F, t22107: F, t22111: F, t22116: F, t22119: F, t22122: F, t2394: F, t2710: F, t274: F, t4068: F, t807: F, t8948: F, t8963: F, t9609: F) -> F {
    let t22127 = t21373 * t278;
    let t22134 = F::cast_from(0.17557713923258613e0_f64) * t21821 * t274 - F::cast_from(0.35115427846517226e0_f64) * t4068 * t22096 + F::cast_from(0.33205381699090447729e-3_f64) * t8948 * t22100 + F::cast_from(0.23410285231011484e0_f64) * t21825 * t274 - F::cast_from(0.79692916077817074549e-2_f64) * t2014 * t22107 - t19167 - F::cast_from(0.8854768453090786061e-3_f64) * t8963 * t22111 + F::cast_from(0.72343824494974941953e-3_f64) * t2014 * t22116 - F::cast_from(0.5116527820486904976e-1_f64) * t10339 * t22119 + F::cast_from(0.959348966341294683e-1_f64) * t2710 * t22122 - F::cast_from(0.25159457085530922489e-1_f64) * t9609 * t22119 - F::cast_from(0.532971647967385935e-1_f64) * t807 * t22127 + F::cast_from(0.41932428475884870816e-1_f64) * t2394 * t22122 - F::cast_from(0.91641760171536135284e-3_f64) * t10355 * t22119;
    t22134
}
