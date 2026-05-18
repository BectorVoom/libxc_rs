//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1038/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1038<F: Float>(t224: F, t2263: F, t23573: F, t23682: F, t216: F, t2371: F, t2414: F, t24021: F, t256: F, t23801: F, t243: F, t2491: F, t2516: F) -> (F, F, F, F, F, F, F) {
    let t24657 = F::new(1.0) / t224 / t2263;
    let t24658 = t24657 * t23573;
    let t24678 = F::new(0.18467901234567901234e0) * t23682;
    let t24699 = t216 / t2414 / t2371;
    let t24733 = t256 * t24021;
    let t24776 = F::new(0.17757530864197530864e0) * t23682;
    let t24795 = t256 * t23801;
    let t24804 = t243 / t2516 / t2491;
    (t24658, t24678, t24699, t24733, t24776, t24795, t24804)
}
