//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 519/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk519<F: Float>(t2996: F, t3001: F, t1030: F, t2995: F, t6: F, t681: F, t134: F, t567: F, t2998: F) -> (F, F, F, F, F, F) {
    let t3002 = t2996 * t3001;
    let t3004 = t1030 * t2995;
    let t3005 = t681 * t6;
    let t3006 = t134 * t567;
    let t3007 = t3005 * t3006;
    let t3008 = t2998 * t3007;
    (t3002, t3004, t3005, t3006, t3007, t3008)
}
