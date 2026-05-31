//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 870/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk870<F: Float>(t1039: F, t1042: F, t232: F, t8595: F, t1043: F, t3666: F, t247: F, t254: F, t257: F, t285: F, t242: F, t2786: F, t30: F) -> (F, F, F, F) {
    let t8815 = t1039 * t1039;
    let t8818 = t1042 * t1042;
    let t8822 = F::cast_from(24955.7003795058_f64) * t232 / t8815 * t8595 / t8818;
    let t8830 = F::cast_from(578.9512619529313_f64) * t3666 * t8595 * t1043;
    let t8834 = F::cast_from(24.0_f64) * t247 * t254 * t257 * t285;
    let t8837 = F::cast_from(0.011483599538271605_f64) * t30 * t2786 * t242;
    (t8822, t8830, t8834, t8837)
}
