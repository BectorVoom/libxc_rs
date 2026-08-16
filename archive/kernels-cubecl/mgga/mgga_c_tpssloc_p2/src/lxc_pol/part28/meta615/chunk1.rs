//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1931/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1931<F: Float>(t16366: F, t22833: F, t16370: F, t26257: F, t3872: F, t1831: F, t80869: F, t22783: F, t5314: F, t26297: F, t80853: F, t80855: F) -> (F, F, F, F, F, F) {
    let t91128 = t22833 * t16366;
    let t91130 = t22833 * t16370;
    let t91133 = t26257 * t3872;
    let t91135 = t80869 * t1831;
    let t91137 = t22783 * t5314;
    let t91140 = t80853 * t80855 * t26297;
    (t91128, t91130, t91133, t91135, t91137, t91140)
}
