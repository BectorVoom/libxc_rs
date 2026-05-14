//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 780/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk780<F: Float>(t6548: F, t6633: F, t393: F, t1820: F, t5036: F, t3330: F, t143: F, t6432: F, t3399: F, t3400: F, t6272: F, t1154: F, t1646: F, t5153: F, t3410: F, t1155: F, t6276: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6634 = t6548 + t6633;
    let t6635 = t6634 * t393;
    let t6637 = 2.0 * t5036 * t1820;
    let t6638 = t1820 * t1820;
    let t6640 = 2.0 * t3330 * t6638;
    let t6641 = t6432 * t143;
    let t6661 = t3399 * t3400 * t6272;
    let t6665 = t1154 * t5153 * t1646;
    let t6669 = t1154 * t3410 * t6272;
    let t6673 = t1154 * t1155 * t6276;
    (t6634, t6635, t6637, t6638, t6640, t6641, t6661, t6665, t6669, t6673)
}
