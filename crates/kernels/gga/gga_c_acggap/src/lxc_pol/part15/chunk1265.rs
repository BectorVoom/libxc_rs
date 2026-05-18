//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1265/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1265<F: Float>(t36327: F, t36333: F, t37960: F, t40490: F, t40493: F, t40497: F, t40501: F, t40505: F, t40507: F, t40511: F, t40515: F, t40517: F, t40519: F, t40521: F, t40523: F, t40525: F, t40527: F, t40529: F) -> F {
    let t42150 = F::new(0.31448092289604152069e-3) * t40490 + F::new(0.25724410870841842183e-2) * t40493 + F::new(0.42874018118069736972e-3) * t40497 + F::new(0.21437009059034868486e-2) * t40501 + F::new(0.12862205435420921092e-2) * t40505 - F::new(0.24009450146119052705e-1) * t40507 - F::new(0.37737710747524982484e-2) * t40511 + F::new(0.12579236915841660828e-2) * t40515 - F::new(0.13719685797782315831e-1) * t40517 - F::new(0.13719685797782315831e-1) * t40519 - F::new(0.68598428988911579156e-2) * t40521 + F::new(0.68598428988911579156e-2) * t40523 - F::new(0.37737710747524982482e-1) * t36327 - F::new(0.17149607247227894789e-2) * t40525 - t37960 - F::new(0.17149607247227894789e-2) * t40527 + F::new(0.25724410870841842184e-1) * t36333 - F::new(0.68598428988911579156e-2) * t40529;
    t42150
}
