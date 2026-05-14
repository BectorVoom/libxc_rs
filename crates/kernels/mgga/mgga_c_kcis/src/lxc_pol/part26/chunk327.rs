//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 327/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk327<F: Float>(t1386: F, t1943: F, t1396: F, t1889: F, t1395: F, t1394: F, t1650: F, t518: F) -> (F, F, F, F, F) {
    let t1944 = t1943 * t1386;
    let t1947 = t1396 * t1889;
    let t1948 = t1395 * t1947;
    let t1949 = t1394 * t1948;
    let t1951 = t518 * t1650;
    (t1944, t1947, t1948, t1949, t1951)
}
