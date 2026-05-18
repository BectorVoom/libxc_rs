//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 751/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk751<F: Float>(t145: F, t1772: F, t301: F, t336: F, t1795: F, t429: F, t1049: F, t1765: F, t1713: F, t3132: F, t345: F, t1298: F, t495: F) -> (F, F, F, F, F, F, F, F) {
    let t5630 = t1772 * t145;
    let t5632 = t336 * t5630 * t301;
    let t5636 = t336 * t429 * t1795;
    let t5639 = t1049 * t1765;
    let t5641 = t1713 * t301;
    let t5642 = t3132 * t5641;
    let t5643 = t345 * t5642;
    let t5645 = t495 * t1298;
    (t5630, t5632, t5636, t5639, t5641, t5642, t5643, t5645)
}
