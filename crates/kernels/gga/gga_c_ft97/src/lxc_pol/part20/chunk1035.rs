//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1035/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1035<F: Float>(t6300: F, t8232: F, t1882: F, t24946: F, t1503: F, t3281: F, t2770: F, t6353: F, t24951: F, t6304: F, t2: F, t25135: F, t458: F, t6307: F) -> (F, F, F, F, F, F, F, F) {
    let t99199 = t8232 * t6300;
    let t99219 = t1882 * t24946;
    let t99229 = 28.0 / 81.0 * t3281 * t1503;
    let t99238 = t2770 * t6353;
    let t99260 = t1882 * t24951;
    let t99271 = t8232 * t6304;
    let t99273 = t2 * t25135;
    let t99312 = t6307 * t458;
    (t99199, t99219, t99229, t99238, t99260, t99271, t99273, t99312)
}
