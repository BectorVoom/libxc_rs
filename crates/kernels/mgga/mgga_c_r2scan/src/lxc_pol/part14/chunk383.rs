//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 383/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk383<F: Float>(t1358: F, t1360: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1519: F, t1522: F, t1526: F, t1529: F, t1533: F) -> F {
    let t1534 = t1358 + t1360 - t1470 + t1519 + t1488 + t1480 - t1459 - t1522 + t1526 - t1463 + t1529 + t1533;
    t1534
}
