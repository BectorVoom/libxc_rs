//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 791/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk791<F: Float>(t3861: F, t5546: F, t1897: F, t3873: F, t1319: F, t1324: F, t5481: F, t1903: F, t659: F, t3883: F, t5427: F, t26: F, t1330: F, t5441: F, t5477: F, t4714: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5548 = 2.0 * t3861 * t5546;
    let t5556 = t3873 * t1897;
    let t5557 = t5556 * t1319;
    let t5559 = t1324 * t5481;
    let t5562 = t659 * t1903;
    let t5564 = t3883 * t5427;
    let t5565 = t26 * t5564;
    let t5567 = t1330 * t5441;
    let t5568 = t26 * t5567;
    let t5570 = t1330 * t5477;
    let t5571 = t4714 * t5570;
    (t5548, t5556, t5557, t5559, t5562, t5564, t5565, t5567, t5568, t5570, t5571)
}
