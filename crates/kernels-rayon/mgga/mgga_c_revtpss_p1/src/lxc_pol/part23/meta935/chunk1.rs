//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3075/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075(t1196: f64, t5184: f64, t68680: f64, t1187: f64, t6534: f64, t1757: f64, t58708: f64, t20400: f64, t5198: f64, t20887: f64, t5192: f64, t58665: f64) -> (f64, f64, f64, f64, f64) {
    let t81322 = 0.51947577317044391277e2_f64 * t1196 * t68680 * t5184;
    let t81323 = t6534 * t1187;
    let t81326 = 0.10526802520742363173e2_f64 * t58708 * t1757 * t81323;
    let t81328 = 0.35089341735807877242e1_f64 * t20400 * t5198;
    let t81330 = 0.35089341735807877242e1_f64 * t5192 * t20887;
    let t81333 = 0.31168546390226634766e3_f64 * t58665 * t5184 * t81323;
    (t81322, t81326, t81328, t81330, t81333)
}
