//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 722/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk722(t429: f64, t7307: f64, t1819: f64, t2042: f64, t2000: f64, t471: f64, t6196: f64, t463: f64, t7066: f64, t6319: f64, t6325: f64, t6464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7308 = t7307 * t429;
    let t7309 = t1819 * t7308;
    let t7310 = t7309 * t2042;
    let t7312 = t471 * t2000;
    let t7313 = t7312 * t6196;
    let t7321 = t463 * t7066;
    let t7324 = 0.10237773105191754_f64 * t6319;
    let t7325 = 0.06825182070127836_f64 * t6325;
    let t7326 = 0.02275060690042612_f64 * t6464;
    (t7308, t7310, t7312, t7313, t7321, t7324, t7325, t7326)
}
