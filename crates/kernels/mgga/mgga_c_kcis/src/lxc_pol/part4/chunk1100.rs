//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1100/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1100<F: Float>(t1319: F, t16004: F, t5457: F, t3809: F, t5458: F, t11633: F, t1961: F, t3762: F, t3814: F, t3767: F, t1897: F, t3780: F, t518: F, t5481: F, t1419: F, t3786: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16005 = t16004 * t1319;
    let t16006 = t5457 * t16005;
    let t16009 = t5458 * t3809;
    let t16010 = t5457 * t16009;
    let t16013 = t11633 * t1961;
    let t16014 = t16013 * t3762;
    let t16017 = t3814 * t1961;
    let t16018 = t16017 * t3767;
    let t16021 = t3780 * t1897;
    let t16022 = t16021 * t3762;
    let t16025 = t11633 * t1897;
    let t16026 = t16025 * t3767;
    let t16029 = t518 * t5481;
    let t16030 = t16029 * t1419;
    let t16031 = t3786 * t16030;
    (t16005, t16006, t16009, t16010, t16014, t16018, t16022, t16026, t16030, t16031)
}
