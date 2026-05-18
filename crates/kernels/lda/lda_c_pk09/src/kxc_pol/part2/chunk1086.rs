//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1086/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1086<F: Float>(t11883: F, t11897: F, t467: F, t452: F, t1971: F, t2825: F, t132: F, t2824: F, t93: F, t2070: F, t2758: F, t11129: F, t477: F) -> (F, F, F, F, F) {
    let t11898 = t11883 + t11897;
    let t11899 = t467 * t11898;
    let t11900 = t11899 * t452;
    let t11903 = t2825 * t1971;
    let t11906 = t132 * t2824;
    let t11907 = t93 * t11906;
    let t11910 = t2070 * t2758;
    let t11913 = t11129 * t477;
    (t11900, t11903, t11907, t11910, t11913)
}
