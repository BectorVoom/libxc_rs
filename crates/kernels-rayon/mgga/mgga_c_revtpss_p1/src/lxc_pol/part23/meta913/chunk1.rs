//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2945/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945(t23754: f64, t2970: f64, t11528: f64, t23767: f64, t2874: f64, t4632: f64, t6141: f64, t11409: f64, t11450: f64, t11466: f64, t15350: f64, t15413: f64, t19282: f64, t19300: f64, t19304: f64, t19307: f64, t19311: f64, t23705: f64, t23755: f64, t23776: f64, t2943: f64, t2968: f64, t41667: f64, t41740: f64, t41742: f64, t4669: f64, t4690: f64, t4707: f64, t4712: f64, t52642: f64, t6177: f64, t6209: f64, t63997: f64, t64125: f64, t953: f64) -> (f64, f64, f64) {
    let t78165 = t23754 * t2970;
    let t78192 = 6.0_f64 * t11528 * t23767;
    let t78195 = 6.0_f64 * t2874 * t4632 * t6141;
    let t78196 = -0.57895126195293126243e3_f64 * t11409 * t6177 * t4669 - 0.24828486201251232145e5_f64 * t41667 * t23776 * t953 - 2.0_f64 * t2943 * t23755 * t953 + 0.32163958997385070134e2_f64 * t2968 * t78165 * t953 + 0.6207121550312808036e4_f64 * t11450 * t19282 * t4669 + 0.19964560303604640732e6_f64 * t41740 * t23705 * t41742 * t953 - 0.35089341735807877242e1_f64 * t63997 * t4690 + 0.51947577317044391276e2_f64 * t64125 * t4712 - 0.35089341735807877242e1_f64 * t15413 * t19300 + 0.51947577317044391276e2_f64 * t15350 * t19304 + 0.10389515463408878255e3_f64 * t15350 * t19307 + 0.30762056574649219972e4_f64 * t52642 * t19311 - 0.31168546390226634765e3_f64 * t11466 * t6209 * t4707 + t78192 + t78195;
    (t78192, t78195, t78196)
}
