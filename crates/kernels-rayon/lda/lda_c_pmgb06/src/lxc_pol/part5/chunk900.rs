//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 900/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk900(t1354: f64, t2833: f64, t2841: f64, t10506: f64, t1152: f64, t421: f64, t8085: f64, t10512: f64, t418: f64, t1343: f64, t2837: f64, t1334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10802 = t2833 * t2841 * t1354;
    let t10806 = 0.002972565416694299_f64 * t1152 * t10506 * t1354;
    let t10808 = 7.439549289525431e-06_f64 * t8085 * t421;
    let t10811 = 0.007901556131563792_f64 * t418 * t10512 * t421;
    let t10813 = t1343 * t2837 * t421;
    let t10817 = 0.03950778065781896_f64 * t1334 * t2837 * t421;
    (t10802, t10806, t10808, t10811, t10813, t10817)
}
