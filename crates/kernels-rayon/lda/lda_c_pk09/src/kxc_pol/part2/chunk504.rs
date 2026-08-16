//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 504/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk504(t1862: f64, t1864: f64, t1866: f64, t1868: f64, t2733: f64, t2736: f64, t2803: f64, t2807: f64, t1871: f64, t452: f64, t2795: f64, t1784: f64, t1786: f64, t1788: f64, t1790: f64) -> (f64, f64, f64, f64, f64) {
    let t2870 = t1862 - 3.2084841915276807_f64 * t2803 + t1864 + 3.2084841915276807_f64 * t2807 + t1866 - 0.64_f64 * t2733 + t1868 + 0.64_f64 * t2736;
    let t2871 = t2870 * t1871;
    let t2872 = t2871 * t452;
    let t2877 = t2795 * t452;
    let t2888 = t1784 - 2.0_f64 * t2803 + t1786 + 2.0_f64 * t2807 + t1788 - 0.505765839233979_f64 * t2733 + t1790 + 0.505765839233979_f64 * t2736;
    (t2870, t2871, t2872, t2877, t2888)
}
