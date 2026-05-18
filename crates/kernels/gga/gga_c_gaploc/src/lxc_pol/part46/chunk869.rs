//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 869/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk869<F: Float>(t20671: F, t31047: F, t34814: F, t26984: F, t9294: F, t1424: F, t2875: F, t544: F, t9065: F, t20368: F, t41596: F, t20367: F, t4820: F) -> (F, F, F, F, F) {
    let t42187 = t31047 * t20671 * t34814;
    let t42188 = F::new(0.42603251059911944084e0) * t42187;
    let t42189 = t26984 * t9294;
    let t42190 = F::new(0.89376224879626066675e-1) * t42189;
    let t42194 = F::new(0.39722766613167140743e-1) * t544 * t9065 * t2875 * t1424;
    let t42195 = t20368 * t41596;
    let t42198 = F::new(0.23833659967900284446e0) * t20367 * t4820 * t42195;
    (t42188, t42190, t42194, t42195, t42198)
}
