//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 578/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk578<F: Float>(t2136: F, t8659: F, t498: F, t615: F, t236: F, t7231: F, t7230: F, t2084: F, t558: F, t27: F, t2139: F, t1173: F, t2410: F, t674: F, t1997: F, t2004: F, t2412: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8660 = t8659 * t2136;
    let t8666 = t615 * t498;
    let t8667 = t236 * t8666;
    let t8668 = t7231 * t8667;
    let t8669 = t7230 * t8668;
    let t8671 = t2084 * t558;
    let t8672 = t27 * t8671;
    let t8673 = t2139 * t8672;
    let t8675 = t2410 * t1173;
    let t8676 = t8675 * t674;
    let t8677 = t8676 * t1997;
    let t8679 = t2412 * t2004;
    (t8660, t8668, t8669, t8672, t8673, t8675, t8676, t8677, t8679)
}
