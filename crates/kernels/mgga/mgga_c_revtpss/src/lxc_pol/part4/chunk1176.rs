//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1176/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1176<F: Float>(t14593: F, t686: F, t874: F, t1558: F, t2811: F, t2482: F, t122: F, t2723: F, t72: F, t676: F, t836: F, t879: F) -> (F, F, F) {
    let t14596 = F::cast_from(0.19514881078765566038e-1_f64) * t874 * t14593 * t686;
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    let t14600 = t2723 * t72 * t122;
    let t14602 = t14600 * t676 * t836;
    let t14603 = t14598 * t14602;
    let t14605 = t879 * t1558;
    (t14596, t14603, t14605)
}
