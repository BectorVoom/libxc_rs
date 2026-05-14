//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 640/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk640<F: Float>(t6522: F, t6319: F, t6325: F, t6547: F, t6464: F, t1783: F, t1793: F, t747: F, t1847: F, t6805: F, t1468: F, t523: F, t1747: F, t6302: F, t506: F, t1931: F, t6488: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6878 = 0.30249879055454143 * t6522;
    let t6882 = 0.04525483399593904 * t6319;
    let t6889 = 0.03016988933062603 * t6325;
    let t6890 = 0.025208232546211785 * t6547;
    let t6895 = 0.010056629776875343 * t6464;
    let t6907 = t1783 * t747 * t1793;
    let t6911 = t1847 * t6805;
    let t6921 = t523 * t1468;
    let t6922 = t6921 * t1747;
    let t6924 = 37.27051603526593 * t6922 * t6302;
    let t6925 = t506 * t1468;
    let t6926 = t6925 * t1747;
    let t6928 = 9.87466743489671 * t6926 * t6302;
    let t6930 = 3.2915558116322368 * t1931 * t6488;
    (t6878, t6882, t6889, t6890, t6895, t6907, t6911, t6924, t6928, t6930)
}
