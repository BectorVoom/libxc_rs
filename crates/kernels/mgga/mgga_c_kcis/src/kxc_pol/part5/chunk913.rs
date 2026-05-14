//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 913/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk913<F: Float>(t3733: F, t4291: F, t554: F, t556: F, t1527: F, t4121: F, t4248: F, t492: F, t12265: F, t577: F, t1370: F, t4455: F, t1607: F, t3978: F, t1606: F, t4354: F, t597: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12530 = t3733 * t4291;
    let t12534 = 1.0 / t556 / t554;
    let t12564 = t1527 * t4121;
    let t12565 = t12564 * sigma2;
    let t12568 = t4248 * t492;
    let t12575 = t12265 * t577;
    let t12605 = t1370 * t4455;
    let t12617 = t3978 * t1607;
    let t12650 = t1606 * t1606;
    let t12651 = 1.0 / t12650;
    let t12688 = 1.0 / t4354 / t597;
    (t12530, t12534, t12565, t12568, t12575, t12605, t12617, t12651, t12688)
}
