//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2200/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2200(t4724: f64, t6206: f64, t981: f64, t4719: f64, t6227: f64, t1633: f64, t6189: f64) -> (f64, f64, f64, f64) {
    let t23446 = t4724 * t6206;
    let t23448 = 0.35089341735807877242e1_f64 * t981 * t23446;
    let t23450 = 0.51947577317044391276e2_f64 * t4719 * t6227;
    let t23451 = t6189 * t1633;
    (t23446, t23448, t23450, t23451)
}
