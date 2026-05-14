//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1138/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1138<F: Float>(t95151: F, t95154: F, t95187: F, t95190: F, t95205: F, t95207: F, t95245: F, t95252: F, t95269: F, t95289: F, t95304: F, t95320: F, t95322: F, t95356: F, t95368: F, t95370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t96077 = t95151 / 12.0;
    let t96078 = t95154 / 6.0;
    let t96086 = 2.0 / 9.0 * t95187;
    let t96087 = 4.0 / 3.0 * t95190;
    let t96091 = 2.0 * t95205;
    let t96092 = 2.0 / 9.0 * t95207;
    let t96105 = 2.0 / 3.0 * t95245;
    let t96107 = t95252 / 3.0;
    let t96111 = 4.0 / 9.0 * t95269;
    let t96116 = t95289 / 3.0;
    let t96120 = t95304 / 6.0;
    let t96126 = t95320 / 18.0;
    let t96127 = 2.0 / 9.0 * t95322;
    let t96137 = 2.0 / 3.0 * t95356;
    let t96140 = 14.0 / 81.0 * t95368;
    let t96141 = t95370 / 9.0;
    (t96077, t96078, t96086, t96087, t96091, t96092, t96105, t96107, t96111, t96116, t96120, t96126, t96127, t96137, t96140, t96141)
}
