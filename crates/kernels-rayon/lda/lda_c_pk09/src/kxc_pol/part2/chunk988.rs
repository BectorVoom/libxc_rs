//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 988/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk988(t2624: f64, t5555: f64, t306: f64, t1336: f64, t2625: f64, t1625: f64, t10020: f64, t1403: f64, t1629: f64, t2626: f64, t5260: f64, t5262: f64, t5276: f64, t5288: f64, t5290: f64, t5296: f64, t5298: f64, t5305: f64, t5309: f64, t5312: f64, t5316: f64, t5326: f64, t5335: f64, t5337: f64, t5566: f64) -> f64 {
    let t10591 = t2624 * t5555;
    let t10592 = t10591 * t306;
    let t10595 = t2625 * t1336;
    let t10596 = t10595 * t1625;
    let t10598 = t1403 * t10020;
    let t10602 = t5260 + t5262 - t5276 - t5288 - t5290 + t5296 + t5298 + 0.04115066352984959_f64 * t5305 + 4.937333717448355_f64 * t5309 - 4.937333717448355_f64 * t5312 + 1.8805371096875316_f64 * t5316 + 2.2140749178833072_f64 * t2626 * t1629 + 2.2140749178833072_f64 * t10592 * t5566 + 2.2140749178833072_f64 * t10596 - 2.2140749178833072_f64 * t10598 + 0.013716887843283197_f64 * t5326 - t5335 - 0.013716887843283197_f64 * t5337;
    t10602
}
