//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1202/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1202<F: Float>(t100400: F, t22958: F, t5674: F, t100186: F, t22952: F, t22953: F, t26011: F, t379: F, t100386: F, t1564: F, t3281: F, t100390: F, t7793: F, t26012: F, t376: F, t5665: F) -> (F, F, F, F, F, F) {
    let t101751 = t5674 * t22958 * t100400;
    let t101754 = t5674 * t22958 * t100186;
    let t101758 = t22952 * t22953 * t26011 * t379;
    let t101761 = t3281 * t1564 * t100386;
    let t101764 = t3281 * t7793 * t100390;
    let t101767 = t5665 * t376 * t26012;
    (t101751, t101754, t101758, t101761, t101764, t101767)
}
