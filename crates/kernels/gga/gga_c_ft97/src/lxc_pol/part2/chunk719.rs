//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 719/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk719<F: Float>(t12792: F, t12796: F, t10998: F, t3506: F, t11003: F, t12561: F, t24: F, t586: F, t1037: F, t1771: F, t150: F, t358: F, t378: F, t3524: F, t458: F, t12302: F, t2102: F) -> (F, F, F, F, F, F, F, F) {
    let t12797 = t12796 * t12792;
    let t12800 = t3506 * t10998;
    let t12803 = t3506 * t11003;
    let t12807 = t24 * t586 * t12561;
    let t12809 = t1771 * t1037;
    let t12812 = t378 * t150 * t358;
    let t12816 = 2.0 / 3.0 * t458 * t3524;
    let t12817 = t2102 * t12302;
    (t12797, t12800, t12803, t12807, t12809, t12812, t12816, t12817)
}
