//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 509/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk509<F: Float>(t1892: F, t225: F, t561: F, t1437: F, t1883: F, t546: F, t1431: F, t1436: F, t213: F, t820: F) -> (F, F, F) {
    let t1893 = t1892 * t225;
    let t1894 = t1893 * t561;
    let t1897 = t1437 * t1883;
    let t1900 = t546 * t1892;
    let t1903 = -t1431 + t1436 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1897 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t1900;
    (t1893, t1894, t1903)
}
