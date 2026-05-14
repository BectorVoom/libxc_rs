//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 873/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk873<F: Float>(t1423: F, t6556: F, t13712: F, t806: F, t2485: F, t3213: F, t2481: F, t2493: F, t3220: F, t132: F, t1547: F, t2605: F, t4836: F, t802: F, t1554: F, t161: F, t2600: F) -> (F, F, F, F, F, F, F, F) {
    let t16137 = t1423 * t6556;
    let t16144 = t13712 * t806;
    let t16150 = t3213 * t2485;
    let t16152 = t3213 * t2481;
    let t16158 = t3220 * t2493;
    let t16161 = t132 * t1547 * t2605;
    let t16173 = t802 * t4836;
    let t16178 = t161 * t1554 * t2600;
    (t16137, t16144, t16150, t16152, t16158, t16161, t16173, t16178)
}
