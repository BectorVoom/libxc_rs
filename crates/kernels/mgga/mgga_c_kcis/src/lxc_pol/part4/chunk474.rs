//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 474/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk474<F: Float>(t1378: F, t1938: F, t286: F, t1367: F, t1368: F, t1930: F, t1934: F, t493: F, t500: F, t1386: F, t1396: F, t1889: F) -> (F, F, F, F) {
    let t1939 = t1378 * t1938;
    let t1940 = t286 * t1939;
    let t1943 = -t1930 * t500 / F::new(36.0) + t1367 + t1368 * t1934 / F::new(288.0) - t493 * t1940 / F::new(96.0);
    let t1944 = t1943 * t1386;
    let t1947 = t1396 * t1889;
    (t1939, t1943, t1944, t1947)
}
