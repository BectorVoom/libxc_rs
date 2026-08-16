//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 796/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk796(t151: f64, t7991: f64, t143: f64, t3277: f64, t3287: f64, t3292: f64, t3300: f64, t4061: f64, t4072: f64, t4581: f64, t7578: f64, t7590: f64, t7598: f64, t7602: f64, t7962: f64, t7974: f64, t7981: f64, t7989: f64) -> f64 {
    let t7992 = t151 * t7991;
    let t7997 = 3.7610742193750633_f64 * t143 * t7962 + 2.2140749178833072_f64 * t7974 * t4061 + 18.635258017632964_f64 * t4581 * t7590 + 37.27051603526593_f64 * t4581 * t7578 - 2.2140749178833072_f64 * t7981 - 4.4281498357666145_f64 * t4072 * t7598 - 2.2140749178833072_f64 * t4072 * t7602 + 0.04115066352984959_f64 * t7989 + 1.2536914064583544_f64 * t7992 + 1.6183441301295518_f64 * t3277 + 2.427516195194328_f64 * t3287 + t3292 - 1.8805371096875316_f64 * t3300;
    t7997
}
