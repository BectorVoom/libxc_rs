//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 386/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk386<F: Float>(t1417: F, t1947: F, t1044: F, t6: F, t125: F, t611: F, t1418: F, t147: F) -> (F, F, F, F) {
    let t1948 = t1417 * t1947;
    let t1951 = t6 * t1044;
    let t1952 = t1951 * t125;
    let t1953 = t611 * t1952;
    let t1954 = t1418 * t147;
    (t1948, t1952, t1953, t1954)
}
