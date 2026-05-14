//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 694/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk694<F: Float>(t110: F, t2703: F, t360: F, t2707: F, t2695: F, t3615: F, t1282: F, t2448: F, t342: F, t38: F, t2209: F, t776: F, t5788: F, t2229: F, t5774: F, t5785: F, t5787: F, t5791: F, t5795: F, t5797: F, t63: F, t6968: F, t6971: F, t6975: F, t6978: F, t6980: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6983 = t110 * t2703;
    let t6984 = t360 * t6983;
    let t6986 = t110 * t2707;
    let t6987 = t360 * t6986;
    let t6989 = t3615 * t2695;
    let t6996 = t1282 * t2448;
    let t7002 = 17.53815 * t38 * t2703 * t342;
    let t7005 = 11.6921 * t38 * t776 * t2209;
    let t7008 = 5.84605 * t38 * t2707 * t342;
    let t7009 = 1.2991222222222223 * t5788;
    let t7012 = -2.93808 * t6968 + 0.73452 * t6971 - t6975 + t6978 - t360 * t6980 / 2.0 - t6984 / 2.0 + t6987 / 6.0 - 29.3808 * t63 * t6989 * t342 + 11.75232 * t63 * t2229 * t2209 + 5.87616 * t63 * t6996 * t342 - t7002 + t7005 + t7008 - t5774 - t5785 - t5787 - t7009 - 4.0 / 9.0 * t5791 + t5795 - 0.97936 * t5797;
    (t6983, t6984, t6986, t6987, t6989, t6996, t7002, t7005, t7008, t7009, t7012)
}
