//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 610/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk610<F: Float>(t5010: F, t5012: F, t404: F, t4767: F, t1146: F, t130: F, t142: F, t408: F, t339: F, t4280: F, t1214: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t5013 = t5010 * t5012;
    let t5014 = F::cast_from(44.15969676259812_f64) * t5013;
    let t5016 = F::cast_from(1.0788960867530346_f64) * t404 * t4767;
    let t5017 = t1146 * t130;
    let t5018 = t142 * t5017;
    let t5020 = F::cast_from(3.948986526768806_f64) * t408 * t5018;
    let t5022 = t339 * t4280 * t130;
    let t5023 = F::cast_from(3.0001361899701053_f64) * t5022;
    let t5031 = t1214 * t68;
    (t5013, t5014, t5016, t5020, t5022, t5023, t5031)
}
