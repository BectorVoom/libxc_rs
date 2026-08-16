//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 610/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk610(t5010: f64, t5012: f64, t404: f64, t4767: f64, t1146: f64, t130: f64, t142: f64, t408: f64, t339: f64, t4280: f64, t1214: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5013 = t5010 * t5012;
    let t5014 = 44.15969676259812_f64 * t5013;
    let t5016 = 1.0788960867530346_f64 * t404 * t4767;
    let t5017 = t1146 * t130;
    let t5018 = t142 * t5017;
    let t5020 = 3.948986526768806_f64 * t408 * t5018;
    let t5022 = t339 * t4280 * t130;
    let t5023 = 3.0001361899701053_f64 * t5022;
    let t5031 = t1214 * t68;
    (t5013, t5014, t5016, t5020, t5022, t5023, t5031)
}
