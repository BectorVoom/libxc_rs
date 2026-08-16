//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1080/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1080(t11782: f64, t1819: f64, t1947: f64, t2855: f64, t2042: f64, t2846: f64, t2845: f64, t305: f64, t2035: f64, t11679: f64, t451: f64, t11102: f64, t11733: f64, t11767: f64, t11773: f64, t11776: f64, t11778: f64, t1748: f64, t2032: f64, t2053: f64, t2104: f64, t2111: f64, t2114: f64, t2783: f64, t2838: f64, t2847: f64, t2856: f64, t472: f64, t6327: f64, t7324: f64, t7325: f64, t7326: f64) -> f64 {
    let t11783 = t1819 * t11782;
    let t11786 = t2855 * t1947;
    let t11787 = t11786 * t2042;
    let t11789 = t2846 * t1947;
    let t11790 = t11789 * t2042;
    let t11794 = t2845 * t305;
    let t11795 = t2035 * t11794;
    let t11798 = t451 * t11679;
    let t11799 = t11798 * t2042;
    let t11805 = -t2111 * t2783 / 6.0_f64 - t472 * t11733 / 6.0_f64 - t2114 * t11767 / 12.0_f64 + t2104 * t2783 / 6.0_f64 - 0.10237773105191754_f64 * t6327 - t7324 - t7325 + t7326 - 0.14975624337724558_f64 * t11773 - 0.14975624337724558_f64 * t11776 + t11778 / 18.0_f64 + t2856 * t2032 / 6.0_f64 - t11783 * t1748 / 6.0_f64 + t11787 / 6.0_f64 + t11790 / 6.0_f64 + t2847 * t2032 / 6.0_f64 - t11795 * t1748 / 6.0_f64 + t11799 / 6.0_f64 + t2838 * t2032 / 6.0_f64 - t2053 * t11102 / 6.0_f64;
    t11805
}
