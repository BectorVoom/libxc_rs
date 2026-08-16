//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2225/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2225(t23694: f64, t964: f64, t973: f64, t981: f64, t1621: f64, t6157: f64, t954: f64, t23451: f64, t11509: f64, t11507: f64, t15104: f64, t15413: f64, t1622: f64, t19173: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23564: f64, t23567: f64, t2968: f64, t3012: f64, t4647: f64, t6158: f64, t6174: f64, t6190: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23696 = t964 * t23694 * t973;
    let t23698 = 0.5848223622634646207e0_f64 * t981 * t23696;
    let t23705 = t6157 * t1621;
    let t23706 = t23705 * t954;
    let t23711 = t23451 * t973;
    let t23714 = t23694 * t973;
    let t23717 = t23451 * t11509;
    let t23720 = -t23461 - t23463 - t23465 + t23469 - t23549 - t23552 + 3.0_f64 * t19173 * t1622 + 3.0_f64 * t4647 * t6174 + t23564 - t23567 - 6.0_f64 * t15104 * t6158 + 6.0_f64 * t2968 * t23706 - 0.35089341735807877242e1_f64 * t15413 * t6190 + 0.35089341735807877242e1_f64 * t3012 * t23711 + 0.5848223622634646207e0_f64 * t965 * t23714 + 0.10254018858216406658e4_f64 * t11507 * t23717;
    (t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720)
}
