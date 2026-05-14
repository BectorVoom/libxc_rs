//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 459/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk459<F: Float>(t486: F, t1371: F, t1650: F, t1370: F, t1924: F) -> (F, F, F) {
    let t495 = 0.0 < t486;
    let t1933 = t1371 * t1650;
    let t1934 = t1370 * t1933;
    let t1938 = piecewise3(t495, t1924, -t1924);
    (t1933, t1934, t1938)
}
