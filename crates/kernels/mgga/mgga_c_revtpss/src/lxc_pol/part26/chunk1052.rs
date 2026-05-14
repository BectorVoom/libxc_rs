//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1052/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1052<F: Float>(t11064: F, t7427: F, t1940: F, t2071: F, t2072: F, t2257: F, t2403: F, t25211: F, t25215: F, t25446: F, t25452: F, t26425: F, t26581: F, t26585: F, t26590: F, t28472: F, t4541: F, t605: F, t7428: F, t92747: F, t92762: F, t92783: F, t92795: F, t92799: F, t92806: F, t92814: F, t92822: F, t9344: F) -> (F, F) {
    let t95976 = t7427 * t11064;
    let t96016 = 9.0 * t4541 * t2071 * t92806 + 3.0 * t1940 * t95976 * t25446 + 3.0 * t92822 * t2072 + 3.0 * t28472 * t92762 + 3.0 * t1940 * t26590 * t92783 + 9.0 * t2403 * t7428 * t25211 + 9.0 / 2.0 * t2403 * t7428 * t25215 - 3.0 / 2.0 * t1940 * t26585 * t25452 + t1940 * t2071 * t9344 / 2.0 + 3.0 / 2.0 * t1940 * t26581 * t605 + 9.0 * t26425 * t92747 + 9.0 / 2.0 * t2403 * t2071 * t92795 + 9.0 / 2.0 * t2403 * t2071 * t92799 + 3.0 / 2.0 * t1940 * t7428 * t2257 + 3.0 / 2.0 * t2403 * t2071 * t92814;
    (t95976, t96016)
}
