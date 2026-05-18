//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 198/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk198<F: Float>(t1360: F, t165: F, t28: F, t1359: F, t167: F, t574: F, t579: F, t91: F, t26: F) -> (F, F, F, F, F) {
    let t1361 = t1360 * t165;
    let t1362 = t28 * t1361;
    let t1366 = t574 * t167 * t1359;
    let t1368 = t91 * t579;
    let t1369 = t1368 * t26;
    (t1361, t1362, t1366, t1368, t1369)
}
