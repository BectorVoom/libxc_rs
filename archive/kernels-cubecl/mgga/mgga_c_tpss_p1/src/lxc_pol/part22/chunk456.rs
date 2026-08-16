//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 456/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk456<F: Float>(t118: F, t1322: F, t1339: F, t1600: F, t1604: F, t1663: F, t485: F, t488: F, t544: F, t626: F, t3: F, param_d: F) -> (F, F, F) {
    let t1665 = -t118 * t1600 - t1322 * t485 - F::cast_from(2.0_f64) * t1339 * t626 + t1604 * t544 + t1663 * t488;
    let t1666 = t3 * t1665;
    let t1668 = param_d * t1665;
    (t1665, t1666, t1668)
}
