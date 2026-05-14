//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1281/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1281<F: Float>(t2104: F, t5974: F, t9269: F, t179: F, t18199: F, t299: F, t3542: F, t2004: F, t21452: F, t25200: F, t25212: F, t25218: F, t25221: F, t25226: F, t25229: F, t25231: F, t25236: F, t2899: F, t2922: F, t3679: F, t5693: F, t5984: F, t735: F, t7640: F, t7769: F, t9550: F, t9577: F) -> (F,) {
    let t25239 = t2104 * t5974 * t9269;
    let t25248 = t299 * t179 * t18199 * t3542;
    let t25254 = -t25212 / 27.0 + t735 * t9550 / 18.0 + t25218 / 432.0 - 0.17149607247227894789e-2 * t2899 * t25221 * t7769 - 0.11433071498151929859e-2 * t25226 - 0.11433071498151929859e-2 * t25229 + 0.6097638132347695925e-2 * t25231 + 0.91464571985215438874e-2 * t5984 * t9577 + 0.19055119163586549765e-3 * t25236 - 0.57165357490759649296e-3 * t25239 - 0.12862205435420921092e-2 * t2922 * t5693 * t3679 * t7640 - 0.60976381323476959249e-2 * t21452 - 0.28582678745379824649e-3 * t25248 + 0.25724410870841842184e-2 * t299 * t179 * t2004 * t25200;
    (t25254,)
}
