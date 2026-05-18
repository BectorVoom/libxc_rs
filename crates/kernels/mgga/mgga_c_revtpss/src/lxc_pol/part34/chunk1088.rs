//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1088/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1088<F: Float>(t1941: F, t243: F, t2712: F, t64: F, t2710: F, t826: F, t2482: F, t27: F, t7036: F, t2689: F, t7030: F, t2718: F) -> (F, F, F, F, F, F) {
    let t25237 = t1941 * t243;
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25243 = F::new(0.90357964994909313586e-5) * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25253 = t2689 * t7030;
    let t25254 = F::new(0.15244095330869239812e-3) * t25253;
    let t25260 = t2718 * t64;
    (t25237, t25240, t25243, t25245, t25254, t25260)
}
