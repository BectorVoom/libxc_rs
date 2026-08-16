//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2913/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913(t19056: f64, t4632: f64, t19327: f64, t52645: f64, t15416: f64, t6142: f64, t52505: f64, t6110: f64, t11450: f64, t11461: f64, t15241: f64, t19272: f64, t19276: f64, t19304: f64, t23711: f64, t23714: f64, t23785: f64, t2982: f64, t41788: f64, t52440: f64, t52511: f64, t52637: f64, t52837: f64, t52840: f64, t6173: f64, t6190: f64, t6209: f64, t77612: f64, t77622: f64, t953: f64) -> (f64, f64, f64, f64, f64) {
    let t77624 = 3.0_f64 * t19056 * t4632;
    let t77628 = 18.0_f64 * t52645 * t19327;
    let t77634 = 3.0_f64 * t15416 * t6142;
    let t77636 = 6.0_f64 * t52505 * t6110;
    let t77637 = 18.0_f64 * t52840 * t19272 + 0.62071215503128080361e4_f64 * t11450 * t6173 * t15241 * t953 + t77612 - 0.35089341735807877242e1_f64 * t52440 * t6190 + 0.35089341735807877242e1_f64 * t11461 * t23711 + 0.5848223622634646207e0_f64 * t2982 * t23714 - 0.31168546390226634766e3_f64 * t52511 * t19304 - t77622 - t77624 - 0.57895126195293126241e3_f64 * t52837 * t19276 - t77628 + 0.51947577317044391276e2_f64 * t52637 * t6209 - 0.10389515463408878255e3_f64 * t41788 * t23785 - t77634 + t77636;
    (t77624, t77628, t77634, t77636, t77637)
}
