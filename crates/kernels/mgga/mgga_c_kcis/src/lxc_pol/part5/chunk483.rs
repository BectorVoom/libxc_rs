//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 483/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk483<F: Float>(t1629: F, t187: F, t2017: F, t2070: F, t2118: F, t2128: F, t633: F, t449: F, t160: F, t62: F, t209: F, t9: F, t119: F, t32: F, t5: F, t645: F, t88: F) -> (F, F, F, F, F, F) {
    let t2132 = t2017 - t2070 + t187 * (-t1629 * t2128 + t2118 * t633 - t2017 + t2070);
    let t2133 = t449 * t2132;
    let t2150 = t62 * t160;
    let t2194 = t209 * t9;
    let t2302 = 0.14764770444444444444e-2 * t5 * t119 * t32;
    let t2303 = t88 * t645;
    (t2132, t2133, t2150, t2194, t2302, t2303)
}
