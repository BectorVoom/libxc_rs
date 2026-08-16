//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2945/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945<F: Float>(t23754: F, t2970: F, t11528: F, t23767: F, t2874: F, t4632: F, t6141: F, t11409: F, t11450: F, t11466: F, t15350: F, t15413: F, t19282: F, t19300: F, t19304: F, t19307: F, t19311: F, t23705: F, t23755: F, t23776: F, t2943: F, t2968: F, t41667: F, t41740: F, t41742: F, t4669: F, t4690: F, t4707: F, t4712: F, t52642: F, t6177: F, t6209: F, t63997: F, t64125: F, t953: F) -> (F, F, F) {
    let t78165 = t23754 * t2970;
    let t78192 = F::cast_from(6.0_f64) * t11528 * t23767;
    let t78195 = F::cast_from(6.0_f64) * t2874 * t4632 * t6141;
    let t78196 = -F::cast_from(0.57895126195293126243e3_f64) * t11409 * t6177 * t4669 - F::cast_from(0.24828486201251232145e5_f64) * t41667 * t23776 * t953 - F::cast_from(2.0_f64) * t2943 * t23755 * t953 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t78165 * t953 + F::cast_from(0.6207121550312808036e4_f64) * t11450 * t19282 * t4669 + F::cast_from(0.19964560303604640732e6_f64) * t41740 * t23705 * t41742 * t953 - F::cast_from(0.35089341735807877242e1_f64) * t63997 * t4690 + F::cast_from(0.51947577317044391276e2_f64) * t64125 * t4712 - F::cast_from(0.35089341735807877242e1_f64) * t15413 * t19300 + F::cast_from(0.51947577317044391276e2_f64) * t15350 * t19304 + F::cast_from(0.10389515463408878255e3_f64) * t15350 * t19307 + F::cast_from(0.30762056574649219972e4_f64) * t52642 * t19311 - F::cast_from(0.31168546390226634765e3_f64) * t11466 * t6209 * t4707 + t78192 + t78195;
    (t78192, t78195, t78196)
}
