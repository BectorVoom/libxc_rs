//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 388/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk388<F: Float>(t51: F, t1225: F, t1228: F, t1368: F, t53: F, t1367: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t1374 = piecewise3(t52, 0.0, 4.0 / 9.0 * t1368 * t1225 + 4.0 / 3.0 * t53 * t1228);
    let t1375 = t1367 + t1374;
    (t1375,)
}
