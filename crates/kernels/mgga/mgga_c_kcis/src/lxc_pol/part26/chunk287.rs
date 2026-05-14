//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 287/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk287<F: Float>(t1552: F, t578: F, t1530: F, t1536: F, t1540: F, t1544: F, t1549: F) -> (F, F) {
    let t1553 = t578 * t1552;
    let t1555 = t1530 / 16.0 - t1536 / 16.0 + t1540 / 24.0 - t1544 / 256.0 + t1549 / 256.0 - t1553 / 192.0;
    (t1553, t1555)
}
