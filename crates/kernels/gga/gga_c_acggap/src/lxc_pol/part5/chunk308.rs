//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 308/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk308<F: Float>(t301: F, t355: F, t721: F, t1060: F, t346: F, t839: F, t345: F, t130: F, t39: F, t14: F, t25: F) -> (F, F, F, F, F, F) {
    let t1061 = t355 * t301;
    let t1062 = t1061 * t721;
    let t1063 = t1060 * t1062;
    let t1065 = t346 * t839;
    let t1066 = t345 * t1065;
    let t1068 = t130 * t39;
    let t1072 = F::new(1.0) / t14 / t25 / F::new(4.0);
    (t1062, t1063, t1065, t1066, t1068, t1072)
}
