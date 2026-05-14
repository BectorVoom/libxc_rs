//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 525/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk525<F: Float>(t193: F, t3718: F, t89: F, t2382: F, t688: F, t2379: F, t223: F, t226: F, t1095: F, t236: F, t1096: F, t709: F, t680: F, t2394: F, t1092: F, t458: F) -> (F, F, F, F, F, F, F, F) {
    let t3720 = t89 * t193 * t3718;
    let t3722 = t688 * t2382;
    let t3723 = t2379 * t3722;
    let t3724 = t223 * t226;
    let t3725 = t236 * t1095;
    let t3726 = t3724 * t3725;
    let t3729 = t1096 * t709;
    let t3730 = t680 * t3729;
    let t3733 = t1096 * t688;
    let t3734 = t2394 * t3733;
    let t3738 = t458 * t1092;
    (t3720, t3723, t3724, t3726, t3730, t3733, t3734, t3738)
}
