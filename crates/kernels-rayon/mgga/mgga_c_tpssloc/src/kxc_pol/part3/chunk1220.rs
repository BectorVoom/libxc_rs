//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1220/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1220(t5230: f64, t68: f64, t12240: f64, t5335: f64, t1352: f64, t16040: f64, t12189: f64, t1804: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12197: f64, t12200: f64, t12205: f64, t12209: f64, t12212: f64, t12228: f64) -> (f64, f64, f64, f64) {
    let t16060 = t5230 * t68;
    let t16065 = t5335 * t12240;
    let t16068 = t16040 * t1352;
    let t16078 = t12189 * t1804;
    let t16080 = -t12188 - 0.25925925925925925926e-1_f64 * t12190 - t12194 + t12196 + 0.38888888888888888888e-2_f64 * t12197 - 0.10555555555555555555e-1_f64 * t12200 - 0.25e-2_f64 * t12205 + 0.83333333333333333332e-3_f64 * t12209 - 0.11666666666666666666e-1_f64 * t12212 + 0.33333333333333333332e-2_f64 * t12228 - 0.12962962962962962962e-1_f64 * t16078;
    (t16060, t16065, t16068, t16080)
}
