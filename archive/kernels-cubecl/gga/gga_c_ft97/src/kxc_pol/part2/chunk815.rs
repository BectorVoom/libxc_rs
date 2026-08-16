//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 815/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk815<F: Float>(t3424: F, t363: F, t12791: F, t1557: F, t586: F, t10998: F, t3506: F, t11003: F, t12561: F, t24: F, t1037: F, t1771: F) -> (F, F, F, F, F, F) {
    let t12792 = t3424 * t363;
    let t12793 = t12791 * t12792;
    let t12796 = t586 * t1557;
    let t12797 = t12796 * t12792;
    let t12800 = t3506 * t10998;
    let t12803 = t3506 * t11003;
    let t12807 = t24 * t586 * t12561;
    let t12809 = t1771 * t1037;
    (t12793, t12797, t12800, t12803, t12807, t12809)
}
