//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2053/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2053<F: Float>(t2118: F, t6936: F, t104062: F, t111304: F, t111345: F, t111390: F, t1456: F, t1458: F, t1464: F, t1914: F, t1921: F, t2111: F, t22533: F, t22571: F, t28945: F, t28993: F, t3: F, t30627: F, t30663: F, t575: F, t5790: F, t5808: F, t6937: F, t7560: F, t8114: F, t8130: F) -> F {
    let t111405 = t6936 * t2118;
    let t111407 = t3 * t111304 * t575 + t1458 * (t111345 + t111390) + t1456 * t30663 + t22533 * t2118 + F::new(2.0) * t8114 * t5808 + t30627 * t1464 + F::new(2.0) * t5790 * t8130 + F::new(2.0) * t28945 * t1921 + F::new(2.0) * t1914 * t28993 + t2111 * t22571 + t104062 + t111405 + t6937 * t7560;
    t111407
}
