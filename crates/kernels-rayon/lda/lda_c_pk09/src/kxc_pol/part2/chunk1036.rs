//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1036/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1036(t11218: f64, t1905: f64, t1948: f64, t6813: f64, t6814: f64, t1240: f64, t2730: f64, t551: f64, t6825: f64, t10964: f64, t6818: f64, t11208: f64, t11211: f64, t11216: f64, t2783: f64, t6493: f64, t6575: f64, t6581: f64, t6589: f64, t6592: f64, t6594: f64, t6598: f64, t6603: f64, t6604: f64, t6606: f64, t6978: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11219 = t1905 * t11218;
    let t11220 = t1948 * t11219;
    let t11223 = t6813 * t6814 * t11218;
    let t11225 = t2730 * t1240;
    let t11226 = t551 * t11225;
    let t11227 = t6825 * t11226;
    let t11229 = t6814 * t10964;
    let t11230 = t6818 * t11229;
    let t11235 = -0.6268457032291772_f64 * t6493 + 2.9824072957409817_f64 * t6978 * t2783 - t6575 - t6581 - t6589 - t6592 + 3.600163427964126_f64 * t6594 - 44.15969676259812_f64 * t11208 + 10.80049028389238_f64 * t11211 - 16.20073542583857_f64 * t11216 - 22.07984838129906_f64 * t11220 - 10.80049028389238_f64 * t11223 + 10.80049028389238_f64 * t11227 + 10.80049028389238_f64 * t11230 + 0.013716887843283197_f64 * t6598 + t6603 - 0.6268457032291772_f64 * t6604 - 6.496391258193384_f64 * t6606;
    (t11220, t11223, t11225, t11227, t11230, t11235)
}
