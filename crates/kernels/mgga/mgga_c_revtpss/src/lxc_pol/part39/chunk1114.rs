//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1114/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1114<F: Float>(t122: F, t2723: F, t72: F, t676: F, t836: F, t14598: F, t1558: F, t879: F, t2482: F, t2801: F, t10443: F, t10552: F, t10554: F, t14312: F, t14313: F, t14315: F, t14317: F, t14324: F, t14327: F, t14329: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F) {
    let t14600 = t2723 * t72 * t122;
    let t14602 = t14600 * t676 * t836;
    let t14603 = t14598 * t14602;
    let t14605 = t879 * t1558;
    let t14606 = t2482 * t14605;
    let t14608 = 0.19514881078765566038e-1 * t14606 * t2801;
    let t14609 = t14312 + t14313 - t9278 + t9308 + t9316 + t10443 + t9329 + t9333 + t14315 + t14317 - t10552 + t10554 - t14324 + t14327 + t14329;
    (t14603, t14608, t14609)
}
