//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1069/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1069<F: Float>(t233: F, t4469: F, t869: F, t689: F, t2435: F, t4519: F, t1558: F, t2723: F, t836: F, t10529: F, t2782: F, t72: F, t686: F, t874: F, t2811: F, t2482: F) -> (F, F, F, F, F, F) {
    let t14574 = t233 * t4469;
    let t14575 = t869 * t14574;
    let t14577 = 0.10975748638225852664e-1 * t689 * t14575;
    let t14581 = t2435 * t4519;
    let t14586 = t1558 * t2723;
    let t14587 = t14586 * t836;
    let t14588 = t10529 * t14587;
    let t14590 = 0.21951497276451705328e-1 * t2782 * t14588;
    let t14593 = t4469 * t72;
    let t14596 = 0.19514881078765566038e-1 * t874 * t14593 * t686;
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    (t14577, t14581, t14586, t14590, t14596, t14598)
}
