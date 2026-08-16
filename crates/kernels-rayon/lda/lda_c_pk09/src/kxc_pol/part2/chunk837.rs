//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 837/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk837(t119: f64, t8049: f64, t200: f64, t7693: f64, t2192: f64, t61: f64, t650: f64, t891: f64, t3772: f64, t7608: f64, t3744: f64, t3750: f64, t7578: f64, t7590: f64, t8517: f64, t8519: f64, t8521: f64, t8525: f64, t8527: f64, t8529: f64, t8531: f64) -> (f64, f64) {
    let t8533 = t119 * t8049;
    let t8535 = t200 * t7693;
    let t8537 = t61 * t2192;
    let t8539 = t891 * t8537 * t650;
    let t8542 = t3772 * t7608;
    let t8548 = -3.600163427964126_f64 * t8517 - 3.600163427964126_f64 * t8519 - 22.07984838129906_f64 * t8521 - 5.40024514194619_f64 * t8525 - 3.600163427964126_f64 * t8527 + 3.600163427964126_f64 * t8529 - 3.600163427964126_f64 * t8531 - 22.07984838129906_f64 * t8533 + 1.6183441301295518_f64 * t8535 - 1.1846959580306418_f64 * t3744 * t8539 - 2.427516195194328_f64 * t8542 - 2.427516195194328_f64 * t3750 * t7590 - 4.855032390388656_f64 * t3750 * t7578;
    (t8533, t8548)
}
