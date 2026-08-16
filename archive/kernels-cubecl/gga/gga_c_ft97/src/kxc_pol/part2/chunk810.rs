//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 810/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk810<F: Float>(t11437: F, t12724: F, t12723: F, t1651: F, t3445: F, t2221: F, t1643: F, t9115: F, t2157: F, t920: F, t2211: F, t2210: F) -> (F, F, F, F) {
    let t12725 = t12724 * t11437;
    let t12726 = t12723 * t12725;
    let t12729 = t3445 * t1651;
    let t12730 = t2221 * t12729;
    let t12733 = t3445 * t1643;
    let t12734 = t9115 * t12733;
    let t12737 = t920 * t2157;
    let t12738 = t2211 * t12737;
    let t12739 = t2210 * t12738;
    (t12726, t12730, t12734, t12739)
}
