//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1089/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1089<F: Float>(t101504: F, t26692: F, t22632: F, t23732: F, t26639: F, t23774: F, t26643: F, t100519: F, t23701: F, t100522: F, t1008: F, t397: F, t1354: F, t3347: F, t3379: F, t58: F) -> (F, F, F, F, F, F, F, F) {
    let t104792 = 0.22226000364197530866e-1 * t26692 * t101504;
    let t104868 = 0.13335600218518518519e0 * t23732 * t22632 * t26639;
    let t104878 = 0.20003400327777777778e0 * t23774 * t22632 * t26643;
    let t104884 = 0.26853068634149852184e-1 * t23701 * t100519;
    let t104888 = t26692 * t100522;
    let t104915 = t397 * t1008;
    let t104920 = t3347 * t1354;
    let t104923 = t58 * t3379;
    (t104792, t104868, t104878, t104884, t104888, t104915, t104920, t104923)
}
