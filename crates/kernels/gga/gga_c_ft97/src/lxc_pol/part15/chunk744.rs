//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 744/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk744<F: Float>(t274: F, t4977: F, t21130: F, t683: F, t1095: F, t231: F, t10327: F, t992: F, t19168: F, t801: F, t278: F, t1193: F, t21373: F, t10339: F, t10355: F, t19167: F, t2014: F, t21821: F, t21825: F, t2394: F, t2710: F, t4068: F, t807: F, t8948: F, t8963: F, t9609: F) -> (F, F, F, F, F, F, F) {
    let t22096 = t274 * t4977;
    let t22100 = t683 * t21130 * t274;
    let t22107 = t231 * t4977 * t1095 * t274;
    let t22110 = t10327 * t992;
    let t22111 = t19168 * t22110;
    let t22116 = t231 * t21130 * t801 * t274;
    let t22119 = t21130 * t278;
    let t22122 = t1193 * t4977;
    let t22127 = t21373 * t278;
    let t22134 = 0.17557713923258613e0 * t21821 * t274 - 0.35115427846517226e0 * t4068 * t22096 + 0.33205381699090447729e-3 * t8948 * t22100 + 0.23410285231011484e0 * t21825 * t274 - 0.79692916077817074549e-2 * t2014 * t22107 - t19167 - 0.8854768453090786061e-3 * t8963 * t22111 + 0.72343824494974941953e-3 * t2014 * t22116 - 0.5116527820486904976e-1 * t10339 * t22119 + 0.959348966341294683e-1 * t2710 * t22122 - 0.25159457085530922489e-1 * t9609 * t22119 - 0.532971647967385935e-1 * t807 * t22127 + 0.41932428475884870816e-1 * t2394 * t22122 - 0.91641760171536135284e-3 * t10355 * t22119;
    (t22096, t22100, t22107, t22110, t22111, t22116, t22134)
}
