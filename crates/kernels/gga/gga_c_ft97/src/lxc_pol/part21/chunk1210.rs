//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1210/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1210<F: Float>(t22940: F, t4589: F, t1882: F, t29963: F, t29958: F, t29828: F, t29882: F, t103556: F, t103571: F, t103572: F, t103592: F, t103607: F, t116093: F, t116095: F, t11837: F, t26042: F, t3103: F, t446: F, t452: F, t6478: F, t6564: F, t83: F, t942: F) -> (F, F) {
    let t117888 = t22940 * t4589;
    let t117902 = t1882 * t29963;
    let t117904 = t1882 * t29958;
    let t117914 = t1882 * t29828;
    let t117916 = t1882 * t29882;
    let t117918 = -t103556 - t103571 + 16.0 / 27.0 * t103572 - t446 * t83 * t117888 / 3.0 - t103592 - 2.0 * t446 * t83 * t116093 + 4.0 / 3.0 * t446 * t83 * t116095 + 2.0 / 3.0 * t446 * t452 * t11837 * t6478 - 2.0 / 9.0 * t117902 - t117904 / 9.0 - t103607 - 2.0 / 3.0 * t446 * t452 * t26042 * t942 - 2.0 / 3.0 * t446 * t452 * t6564 * t3103 - 2.0 / 9.0 * t117914 - 4.0 / 9.0 * t117916;
    (t117888, t117918)
}
