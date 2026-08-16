//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 810/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk810(t161: f64, t8141: f64, t200: f64, t3667: f64, t3668: f64, t3670: f64, t3744: f64, t7706: f64, t7768: f64, t7776: f64, t7962: f64, t8117: f64, t8121: f64, t8124: f64, t8129: f64, t8131: f64) -> f64 {
    let t8142 = t161 * t8141;
    let t8144 = t3667 + 1.2536914064583544_f64 * t3668 + 1.2536914064583544_f64 * t3670 - 19.489173774580152_f64 * t8117 + 22.07984838129906_f64 * t8121 + 1.1846959580306418_f64 * t3744 * t8124 - 4.738783832122567_f64 * t8129 + 3.2915558116322368_f64 * t8131 + 2.427516195194328_f64 * t200 * t7962 + 2.427516195194328_f64 * t200 * t7768 + 2.427516195194328_f64 * t200 * t7776 + 2.427516195194328_f64 * t200 * t7706 + 3.2915558116322368_f64 * t8142;
    t8144
}
