//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 903/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk903<F: Float>(t23667: F, t27096: F, t5899: F, t590: F, t6615: F, t586: F, t28: F, t5890: F, t2112: F, t26950: F, t1369: F, t26909: F, t376: F, t6657: F, t1969: F, t23652: F, t925: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27120 = t23667 * t27096;
    let t27121 = t5899 * t27120;
    let t27123 = t6615 * t590;
    let t27124 = t586 * t27123;
    let t27126 = t5890 * t28 * t27124;
    let t27128 = t2112 * t26950;
    let t27130 = t1369 * t28 * t27128;
    let t27131 = t2112 * t26909;
    let t27133 = t1369 * t28 * t27131;
    let t27135 = t5890 * t376 * t6657;
    let t27138 = t1969 * t23652 * t925;
    (t27120, t27121, t27124, t27126, t27128, t27130, t27131, t27133, t27135, t27138)
}
