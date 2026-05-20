//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2612/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2612<F: Float>(t1544: F, t1559: F, t40834: F, t854: F, t18413: F, t18525: F, t2661: F, t40693: F, t10726: F, t4366: F, t10886: F, t18608: F, t808: F) -> (F, F, F, F, F) {
    let t61837 = t1559 * t1544;
    let t61839 = t40834 * t854 * t61837;
    let t61860 = t2661 * t40693 * t18413 * t18525;
    let t61864 = t2661 * t10726 * t18413 * t4366;
    let t61877 = t10886 * t808 * t18608;
    (t61837, t61839, t61860, t61864, t61877)
}
