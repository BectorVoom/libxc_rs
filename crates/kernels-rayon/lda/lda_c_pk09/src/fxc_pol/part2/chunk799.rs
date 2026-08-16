//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 799/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk799(t8013: f64, t8026: f64, t760: f64, t772: f64, t4064: f64, t7608: f64, t2210: f64, t2214: f64, t2341: f64, t3303: f64, t3368: f64, t3371: f64, t3475: f64, t3477: f64, t3897: f64, t4072: f64, t7578: f64, t7590: f64, t773: f64, t98: f64) -> f64 {
    let t8027 = t8013 + t8026;
    let t8028 = t760 * t8027;
    let t8029 = t8028 * t772;
    let t8037 = t4064 * t7608;
    let t8045 = -t3303 - 2.9824072957409817_f64 * t773 * t2341 - 2.9824072957409817_f64 * t8029 * t98 - 19.489173774580152_f64 * t3368 - t3371 - 1.6183441301295518_f64 * t3475 - 1.6183441301295518_f64 * t3477 + 2.2140749178833072_f64 * t3897 * t2210 - 2.2140749178833072_f64 * t8037 - 2.2140749178833072_f64 * t4072 * t7590 - 4.4281498357666145_f64 * t4072 * t7578 + 2.2140749178833072_f64 * t3897 * t2214;
    t8045
}
