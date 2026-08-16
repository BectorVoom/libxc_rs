//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 978/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk978<F: Float>(t1397: F, t2674: F, t1240: F, t5624: F, t93: F, t1214: F, t1435: F, t2481: F, t1336: F, t2606: F, t1625: F, t2666: F) -> (F, F, F, F, F, F) {
    let t10435 = t2674 * t1397;
    let t10439 = t2674 * t1240;
    let t10441 = t5624 * t93 * t10439;
    let t10443 = t2674 * t1214;
    let t10447 = t2481 * t1435;
    let t10449 = t2606 * t1336;
    let t10450 = t10449 * t1625;
    let t10454 = t2666 * t1336;
    (t10435, t10441, t10443, t10447, t10450, t10454)
}
