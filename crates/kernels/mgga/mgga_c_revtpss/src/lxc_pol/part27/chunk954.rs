//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 954/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk954<F: Float>(t11114: F, t11118: F, t11399: F, t11404: F, t11409: F, t11411: F, t11445: F, t11450: F, t11453: F, t11456: F, t11461: F, t11466: F, t11468: F, t11502: F, t11507: F, t11510: F, t11513: F, t11517: F, t2938: F, t2943: F, t2963: F, t2968: F, t2971: F, t2982: F, t3007: F, t3015: F, t946: F, t955: F, t965: F, t974: F) -> F {
    let t11520 = t11114 - t11118 + F::new(3.0) * t11399 * t955 + F::new(3.0) * t2938 * t2963 + F::new(0.96491876992155210402e2) * t11404 * t2971 - F::new(0.19298375398431042081e3) * t11409 * t11411 + F::new(1.0) * t946 * t11445 + F::new(0.2069040516770936012e4) * t11450 * t11453 + F::new(0.17544670867903938621e1) * t11456 * t974 + F::new(0.17544670867903938621e1) * t2982 * t3007 + F::new(0.51947577317044391276e2) * t11461 * t3015 - F::new(0.10389515463408878255e3) * t11466 * t11468 + F::new(0.5848223622634646207e0) * t965 * t11502 + F::new(0.10254018858216406658e4) * t11507 * t11510 - F::new(6.0) * t2943 * t11513 + F::new(0.96491876992155210402e2) * t2968 * t11517;
    t11520
}
