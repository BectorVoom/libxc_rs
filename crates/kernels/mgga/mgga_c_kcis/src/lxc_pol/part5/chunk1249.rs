//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1249/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1249<F: Float>(t609: F, t23024: F, t1608: F, t286: F, t25: F, t7493: F, t1599: F, t7430: F, t6141: F, t6168: F, t1612: F, t18223: F, t23158: F, t23174: F, t23178: F, t23182: F, t23186: F, t23192: F, t23194: F, t6179: F, t6185: F) -> (F,) {
    let t614 = 0.0 < t609;
    let t23198 = piecewise3(t614, t23024, -t23024);
    let t23199 = t1608 * t23198;
    let t23200 = t286 * t23199;
    let t23207 = t25 * t7493;
    let t23208 = t1599 * t23207;
    let t23210 = t25 * t7430;
    let t23211 = t1599 * t23210;
    let t23213 = t6141 * t6168;
    let t23215 = t23174 / 1296.0 - t1599 * t23178 / 32.0 + t1599 * t23182 / 48.0 + t1599 * t23186 / 576.0 - t6141 * t6179 / 18.0 - t23192 / 864.0 - t23194 / 324.0 - t18223 / 432.0 - t1599 * t23200 / 192.0 - 11.0 / 216.0 * t23158 * t1612 + t6141 * t6185 / 36.0 - t23208 / 576.0 + t23211 / 288.0 + t23213 / 108.0;
    (t23215,)
}
