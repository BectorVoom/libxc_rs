//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 766/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk766<F: Float>(t24433: F, t9770: F, t446: F, t6061: F, t713: F, t2506: F, t1434: F, t193: F, t1424: F, t2459: F, t6124: F, t681: F, t747: F, t743: F, t6109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24457 = t9770 * t24433;
    let t24458 = t446 * t24457;
    let t24460 = t6061 * t713;
    let t24461 = t2506 * t24460;
    let t24463 = t1434 * t193 * t24461;
    let t24465 = t1424 * t2459;
    let t24466 = t2506 * t24465;
    let t24468 = t1434 * t193 * t24466;
    let t24470 = t1434 * t681 * t6124;
    let t24472 = t6061 * t747;
    let t24473 = t743 * t24472;
    let t24475 = t6109 * t193 * t24473;
    let t24477 = t747 * t713;
    (t24457, t24458, t24460, t24461, t24463, t24465, t24466, t24468, t24470, t24473, t24475, t24477)
}
