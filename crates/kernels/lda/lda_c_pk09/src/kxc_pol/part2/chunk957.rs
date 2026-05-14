//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 957/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk957<F: Float>(t11936: F, t11950: F, t444: F, t2036: F, t11733: F, t11863: F, t11866: F, t11900: F, t11903: F, t11907: F, t11910: F, t11913: F, t11915: F, t1748: F, t2088: F, t2114: F, t2116: F, t2783: F, t455: F, t463: F) -> (F, F) {
    let t11951 = t11936 + t11950;
    let t11952 = t11951 * t444;
    let t11953 = t11952 * t2036;
    let t11956 = t11863 * t2116 / 12.0 + t11866 * t455 / 6.0 + t11900 * t455 / 6.0 + t11903 * t455 / 6.0 + t2114 * t11907 / 12.0 + t11910 * t455 / 6.0 + 0.14975624337724558 * t11913 - t11915 * t1748 / 6.0 + t2088 * t2783 / 6.0 + t463 * t11733 / 6.0 - t11953 * t1748 / 6.0;
    (t11951, t11956)
}
