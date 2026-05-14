//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1175/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1175<F: Float>(t29721: F, t38921: F, t432: F, t446: F, t3103: F, t6469: F, t8411: F, t1564: F, t18: F, t25955: F, t3281: F, t116082: F, t1317: F, t28: F, t469: F, t1586: F, t29569: F) -> (F, F, F, F, F) {
    let t116735 = t446 * t38921 * t29721 * t432;
    let t116739 = t446 * t8411 * t6469 * t3103;
    let t116743 = t3281 * t1564 * t25955 * t18;
    let t116747 = t1317 * t28 * t469 * t116082;
    let t116749 = t1586 * t29569;
    (t116735, t116739, t116743, t116747, t116749)
}
