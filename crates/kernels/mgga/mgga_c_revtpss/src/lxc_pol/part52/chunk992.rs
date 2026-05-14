//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 992/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk992<F: Float>(t1936: F, t28986: F, t572: F, t7553: F, t7741: F, t1918: F, t2040: F, t2115: F, t34011: F, t34014: F, t34341: F, t34346: F, t34348: F, t34350: F, t34358: F, t34362: F, t573: F, t7944: F, t8124: F, t8127: F, t8616: F, t8725: F) -> (F, F, F) {
    let t34363 = t28986 * t1936;
    let t34365 = 6.0 * t572 * t34363;
    let t34366 = t7553 * t7741;
    let t34368 = 6.0 * t572 * t34366;
    let t34369 = 3.0 * t1918 * t8725 + 6.0 * t2040 * t8124 + 3.0 * t2040 * t8127 + 3.0 * t2115 * t7944 + t34341 * t573 + t34011 + t34014 + t34346 + t34348 + t34350 + t34358 + t34362 + t34365 + t34368 + t8616;
    (t34363, t34366, t34369)
}
