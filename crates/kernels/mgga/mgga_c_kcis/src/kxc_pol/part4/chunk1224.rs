//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1224/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1224<F: Float>(t25: F, t494: F, t6178: F, t1599: F, t12651: F, t2104: F, t4457: F, t6176: F, t1369: F, t2470: F, t6164: F, t12615: F, t12664: F, t18184: F, t18188: F, t18192: F, t18197: F, t18201: F, t18205: F, t4435: F, t4439: F, t4442: F, t4451: F, t6141: F) -> (F,) {
    let t18210 = t25 * t494;
    let t18211 = t18210 * t6178;
    let t18213 = t1599 * t18211 / 144.0;
    let t18217 = t12651 * t2104 * t4457;
    let t18218 = t6176 * t18217;
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    let t18223 = t1599 * t18222;
    let t18225 = 7.0 / 1296.0 * t4439 * t18184 - t4439 * t18188 / 108.0 + t18192 * t4442 / 108.0 + t1599 * t18197 / 48.0 + t1599 * t18201 / 96.0 - t18205 - t6141 * t4451 / 216.0 - t6141 * t4435 / 162.0 + t18213 - t12615 / 576.0 + t12664 / 288.0 - t1599 * t18218 / 32.0 + 7.0 / 864.0 * t18223;
    (t18225,)
}
