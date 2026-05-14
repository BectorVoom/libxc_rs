//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1188/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1188<F: Float>(t100293: F, t100295: F, t116414: F, t116417: F, t116420: F, t116423: F, t116427: F, t116431: F, t116434: F, t116436: F, t116440: F, t100305: F, t100308: F, t100310: F, t100312: F, t100314: F, t116444: F, t116448: F, t116453: F, t116456: F, t116460: F, t116462: F, t116467: F) -> (F, F) {
    let t117102 = -4.0 / 9.0 * t116414 + t116417 / 3.0 + 2.0 / 3.0 * t116420 - 2.0 / 9.0 * t116423 + 2.0 / 27.0 * t116427 - 4.0 / 27.0 * t116431 - t116434 / 18.0 + t100293 + t100295 + t116436 / 54.0 - 2.0 / 9.0 * t116440;
    let t117111 = -2.0 / 27.0 * t116444 - 2.0 / 9.0 * t116448 - 2.0 / 27.0 * t100305 - t100308 - t116453 / 9.0 + t116456 / 27.0 - t116460 / 54.0 - t116462 / 81.0 - t116467 / 6.0 + t100310 + t100312 - t100314;
    (t117102, t117111)
}
