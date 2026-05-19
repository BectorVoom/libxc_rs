//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 900/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk900<F: Float>(t1354: F, t2833: F, t2841: F, t10506: F, t1152: F, t421: F, t8085: F, t10512: F, t418: F, t1343: F, t2837: F, t1334: F) -> (F, F, F, F, F, F) {
    let t10802 = t2833 * t2841 * t1354;
    let t10806 = F::cast_from(0.002972565416694299_f64) * t1152 * t10506 * t1354;
    let t10808 = F::cast_from(7.439549289525431e-06_f64) * t8085 * t421;
    let t10811 = F::cast_from(0.007901556131563792_f64) * t418 * t10512 * t421;
    let t10813 = t1343 * t2837 * t421;
    let t10817 = F::cast_from(0.03950778065781896_f64) * t1334 * t2837 * t421;
    (t10802, t10806, t10808, t10811, t10813, t10817)
}
