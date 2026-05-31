//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1035/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1035<F: Float>(t22: F, t7856: F, t2263: F, t2672: F, t10: F, t2666: F, t2662: F, t312: F, t508: F, t2269: F, t116: F, t23533: F, t286: F) -> (F, F, F, F, F, F, F) {
    let t24447 = t22 * t7856;
    let t24468 = t2672 * t2263;
    let t24502 = t2666 * t10;
    let t24503 = t2662 * t24502;
    let t24513 = t508 * t312;
    let t24535 = t2672 * t2269;
    let t24546 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t286 * t116 * t23533;
    (t24447, t24468, t24502, t24503, t24513, t24535, t24546)
}
