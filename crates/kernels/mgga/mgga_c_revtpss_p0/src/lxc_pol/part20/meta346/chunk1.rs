//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1274/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274<F: Float>(t12268: F, t3617: F, t2258: F, t3628: F, t3367: F, t471: F, t2251: F, t17350: F, t3767: F, t1121: F, t1248: F, t606: F) -> (F, F, F, F, F) {
    let t17550 = t3617 * t12268;
    let t17638 = t3628 * t2258;
    let t17643 = t471 * t3367;
    let t17644 = t17643 * t2251;
    let t17654 = t3767 * t17350;
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    (t17550, t17638, t17644, t17654, t17656)
}
