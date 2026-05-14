//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 699/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk699<F: Float>(t11744: F, t1950: F, t1945: F, t5332: F, t10522: F, t642: F, t735: F, t734: F, t10534: F, t5322: F, t5321: F, t1954: F, t5307: F, t1931: F, t5303: F, t5336: F) -> (F, F, F, F, F, F, F) {
    let t11745 = t11744 * t1950;
    let t11747 = t1945 * t5332;
    let t11749 = t642 * t10522;
    let t11750 = t735 * t11749;
    let t11751 = t734 * t11750;
    let t11753 = t5322 * t10534;
    let t11754 = t5321 * t11753;
    let t11756 = t5307 * t1954;
    let t11758 = t1931 * t5303;
    let t11760 = t1945 * t5336;
    (t11745, t11747, t11751, t11754, t11756, t11758, t11760)
}
