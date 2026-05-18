//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 764/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk764<F: Float>(t194: F, t197: F, t8991: F, t10305: F, t10308: F, t10309: F, t10313: F, t10316: F, t10321: F, t10329: F, t10334: F, t10339: F, t10340: F, t10343: F, t2014: F, t2394: F, t2705: F, t2710: F, t274: F, t278: F, t807: F, t8948: F, t8959: F, t8963: F, t9600: F, t9609: F) -> (F, F) {
    let t10355 = t8991 / t197 / t194;
    let t10358 = F::new(0.17557713923258613e0) * t10305 * t274 - F::new(0.35115427846517226e0) * t10308 * t10309 + F::new(0.33205381699090447729e-3) * t8948 * t10313 + F::new(0.23410285231011484e0) * t10316 * t274 - F::new(0.79692916077817074549e-2) * t2014 * t10321 - F::new(0.8854768453090786061e-3) * t8959 * t2705 - F::new(0.8854768453090786061e-3) * t8963 * t10329 + F::new(0.72343824494974941953e-3) * t2014 * t10334 - F::new(0.5116527820486904976e-1) * t10339 * t10340 + F::new(0.959348966341294683e-1) * t2710 * t10343 - F::new(0.25159457085530922489e-1) * t9609 * t10340 - F::new(0.532971647967385935e-1) * t807 * t9600 * t278 + F::new(0.41932428475884870816e-1) * t2394 * t10343 - F::new(0.91641760171536135284e-3) * t10355 * t10340;
    (t10355, t10358)
}
