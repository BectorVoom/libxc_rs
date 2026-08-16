//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1089/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1089(t11936: f64, t11950: f64, t444: f64, t2036: f64, t11733: f64, t11863: f64, t11866: f64, t11900: f64, t11903: f64, t11907: f64, t11910: f64, t11913: f64, t11915: f64, t1748: f64, t2088: f64, t2114: f64, t2116: f64, t2783: f64, t455: f64, t463: f64) -> (f64, f64) {
    let t11951 = t11936 + t11950;
    let t11952 = t11951 * t444;
    let t11953 = t11952 * t2036;
    let t11956 = t11863 * t2116 / 12.0_f64 + t11866 * t455 / 6.0_f64 + t11900 * t455 / 6.0_f64 + t11903 * t455 / 6.0_f64 + t2114 * t11907 / 12.0_f64 + t11910 * t455 / 6.0_f64 + 0.14975624337724558_f64 * t11913 - t11915 * t1748 / 6.0_f64 + t2088 * t2783 / 6.0_f64 + t463 * t11733 / 6.0_f64 - t11953 * t1748 / 6.0_f64;
    (t11951, t11956)
}
