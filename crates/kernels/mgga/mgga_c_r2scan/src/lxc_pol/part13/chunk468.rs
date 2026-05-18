//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 468/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk468<F: Float>(t481: F, t537: F, t495: F, t2124: F, t1625: F, t1629: F, t1635: F, t1638: F, t2080: F, t2083: F, t2088: F, t2095: F, t2108: F, t2119: F, t2122: F, t279: F, t535: F, t574: F) -> (F, F) {
    let t2125 = t537 * t481;
    let t2126 = t2125 * t495;
    let t2127 = t2124 * t2126;
    let t2130 = -F::new(0.27439371595564631661e-1) * t535 * t1625 - F::new(0.27439371595564631661e-1) * t535 * t1629 + F::new(0.23115257973478049502e0) * t1635 - F::new(0.43341108700271342816e-1) * t574 * t1638 + F::new(0.43341108700271342816e-1) * t2080 * t279 - F::new(0.23115257973478049502e0) * t2083 + t2088 + t2095 + t2108 + t2119 + F::new(0.10975748638225852664e0) * t2122 * t2127;
    (t2127, t2130)
}
