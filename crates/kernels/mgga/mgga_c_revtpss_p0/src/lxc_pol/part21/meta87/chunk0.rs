//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 617/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk617<F: Float>(t1883: F, t828: F, t1390: F, t1414: F, t1868: F, t1368: F, t1370: F, t1378: F, t1383: F, t1388: F, t1407: F, t1410: F, t1873: F) -> (F, F, F) {
    let t1884 = t828 * t1883;
    let t1885 = t1390 * t1884;
    let t1889 = t1414 * t828 * t1868;
    let t1892 = -t1368 - t1370 * t1873 / F::new(48.0) - t1378 + t1383 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1885 - t1407 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1889;
    (t1885, t1889, t1892)
}
