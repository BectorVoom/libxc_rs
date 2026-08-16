//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1396/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1396(t18137: f64, t737: f64, t110: f64, t2105: f64, t1599: f64, t18093: f64, t18096: f64, t18100: f64, t18105: f64, t18110: f64, t18116: f64, t18121: f64, t18125: f64, t18130: f64, t18133: f64, t4439: f64) -> f64 {
    let t18138 = t737 * t18137;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18144 = -t18093 - t4439 * t18096 / 288.0_f64 - t4439 * t18100 / 576.0_f64 + t4439 * t18105 / 288.0_f64 - t4439 * t18110 / 432.0_f64 - t1599 * t18116 / 192.0_f64 + t4439 * t18121 / 144.0_f64 - t4439 * t18125 / 576.0_f64 + t4439 * t18130 / 144.0_f64 - t4439 * t18133 / 288.0_f64 - t1599 * t18138 / 288.0_f64 + t18142 / 864.0_f64;
    t18144
}
