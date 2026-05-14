//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 911/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk911<F: Float>(t11225: F, t551: F, t6825: F, t10964: F, t6814: F, t6818: F, t11208: F, t11211: F, t11216: F, t11220: F, t11223: F, t2783: F, t6493: F, t6575: F, t6581: F, t6589: F, t6592: F, t6594: F, t6598: F, t6603: F, t6604: F, t6606: F, t6978: F) -> (F, F, F) {
    let t11226 = t551 * t11225;
    let t11227 = t6825 * t11226;
    let t11229 = t6814 * t10964;
    let t11230 = t6818 * t11229;
    let t11235 = -0.6268457032291772 * t6493 + 2.9824072957409817 * t6978 * t2783 - t6575 - t6581 - t6589 - t6592 + 3.600163427964126 * t6594 - 44.15969676259812 * t11208 + 10.80049028389238 * t11211 - 16.20073542583857 * t11216 - 22.07984838129906 * t11220 - 10.80049028389238 * t11223 + 10.80049028389238 * t11227 + 10.80049028389238 * t11230 + 0.013716887843283197 * t6598 + t6603 - 0.6268457032291772 * t6604 - 6.496391258193384 * t6606;
    (t11227, t11230, t11235)
}
