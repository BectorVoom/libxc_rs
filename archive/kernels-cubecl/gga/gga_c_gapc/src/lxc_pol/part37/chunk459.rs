//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 459/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk459<F: Float>(t2503: F, t2504: F, t604: F, t820: F, t1764: F, t919: F, t2387: F, t282: F, t129: F, t825: F, t869: F) -> (F, F, F, F, F) {
    let t2505 = t2503 * t2504;
    let t2508 = t604 * t820;
    let t2511 = t1764 * t919;
    let t2514 = t2387 * t282;
    let t2515 = t2514 * t129;
    let t2520 = t869 * t825;
    (t2505, t2508, t2511, t2515, t2520)
}
