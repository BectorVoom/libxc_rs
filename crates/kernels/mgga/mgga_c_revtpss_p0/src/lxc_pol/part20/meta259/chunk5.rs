//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1102/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1102<F: Float>(t11467: F, t11509: F, t2962: F, t955: F, t2970: F, t953: F, t11114: F, t11118: F, t11399: F, t11404: F, t11409: F, t11411: F, t11445: F, t11450: F, t11453: F, t11456: F, t11461: F, t11466: F, t11468: F, t11502: F, t11507: F, t2938: F, t2943: F, t2963: F, t2968: F, t2971: F, t2982: F, t3007: F, t3015: F, t946: F, t965: F, t974: F) -> (F, F, F, F) {
    let t11510 = t11467 * t11509;
    let t11513 = t955 * t2962;
    let t11517 = t2962 * t2970 * t953;
    let t11520 = t11114 - t11118 + F::cast_from(3.0_f64) * t11399 * t955 + F::cast_from(3.0_f64) * t2938 * t2963 + F::cast_from(0.96491876992155210402e2_f64) * t11404 * t2971 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t11411 + F::cast_from(1.0_f64) * t946 * t11445 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t11453 + F::cast_from(0.17544670867903938621e1_f64) * t11456 * t974 + F::cast_from(0.17544670867903938621e1_f64) * t2982 * t3007 + F::cast_from(0.51947577317044391276e2_f64) * t11461 * t3015 - F::cast_from(0.10389515463408878255e3_f64) * t11466 * t11468 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t11502 + F::cast_from(0.10254018858216406658e4_f64) * t11507 * t11510 - F::cast_from(6.0_f64) * t2943 * t11513 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t11517;
    (t11510, t11513, t11517, t11520)
}
