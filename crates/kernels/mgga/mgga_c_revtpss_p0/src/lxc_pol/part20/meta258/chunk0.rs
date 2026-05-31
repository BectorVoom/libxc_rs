//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1094/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1094<F: Float>(t11428: F, t11443: F, t954: F, t2966: F, t944: F, t302: F, t2969: F, t310: F, t11410: F, t2979: F, t964: F, t3011: F, t960: F) -> (F, F, F, F, F, F, F, F) {
    let t11444 = t11428 + t11443;
    let t11445 = t11444 * t954;
    let t11449 = F::cast_from(1.0_f64) / t2966 / t944;
    let t11450 = t302 * t11449;
    let t11452 = F::cast_from(1.0_f64) / t2969 / t310;
    let t11453 = t11410 * t11452;
    let t11456 = t2979 * t964;
    let t11461 = t960 * t3011;
    (t11444, t11445, t11449, t11450, t11452, t11453, t11456, t11461)
}
