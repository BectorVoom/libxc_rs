//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 966/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk966(t297: f64, t9973: f64, t1633: f64, t10240: f64, t10244: f64, t10249: f64, t1451: f64, t1562: f64, t1597: f64, t1609: f64, t2587: f64, t328: f64, t5865: f64, t5868: f64, t5871: f64, t5880: f64, t5884: f64) -> f64 {
    let t10251 = t9973 * t297;
    let t10252 = t10251 * t1633;
    let t10255 = t5865 / 6.0_f64 + t5868 / 6.0_f64 + t5871 - t5880 - t5884 - t1597 * t2587 / 6.0_f64 - t328 * t10240 / 6.0_f64 - t1609 * t10244 / 12.0_f64 + t1562 * t2587 / 6.0_f64 - 0.037002892246025966_f64 * t10249 - t10252 * t1451 / 6.0_f64;
    t10255
}
