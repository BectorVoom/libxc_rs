//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 910/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk910<F: Float>(t1811: F, t5219: F, t1284: F, t6564: F, t473: F, t6695: F, t20849: F, t487: F, t5812: F, t602: F, t1469: F, t70: F, t72: F) -> (F, F, F, F, F, F) {
    let t21394 = t5219 * t1811;
    let t21439 = t6564 * t1284;
    let t21541 = t473 * t6695;
    let t21621 = t20849 * t487;
    let t21663 = t5812 * t602;
    let t21686 = t1469 * t70 * t72;
    (t21394, t21439, t21541, t21621, t21663, t21686)
}
