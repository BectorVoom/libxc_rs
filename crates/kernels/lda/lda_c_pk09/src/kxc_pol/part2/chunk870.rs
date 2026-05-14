//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 870/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk870<F: Float>(t2624: F, t5555: F, t306: F, t1336: F, t2625: F, t1625: F, t10020: F, t1403: F, t1629: F, t2626: F, t5260: F, t5262: F, t5276: F, t5288: F, t5290: F, t5296: F, t5298: F, t5305: F, t5309: F, t5312: F, t5316: F, t5326: F, t5335: F, t5337: F, t5566: F) -> (F,) {
    let t10591 = t2624 * t5555;
    let t10592 = t10591 * t306;
    let t10595 = t2625 * t1336;
    let t10596 = t10595 * t1625;
    let t10598 = t1403 * t10020;
    let t10602 = t5260 + t5262 - t5276 - t5288 - t5290 + t5296 + t5298 + 0.04115066352984959 * t5305 + 4.937333717448355 * t5309 - 4.937333717448355 * t5312 + 1.8805371096875316 * t5316 + 2.2140749178833072 * t2626 * t1629 + 2.2140749178833072 * t10592 * t5566 + 2.2140749178833072 * t10596 - 2.2140749178833072 * t10598 + 0.013716887843283197 * t5326 - t5335 - 0.013716887843283197 * t5337;
    (t10602,)
}
