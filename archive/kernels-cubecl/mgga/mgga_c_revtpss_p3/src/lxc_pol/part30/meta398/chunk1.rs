//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1492/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1492<F: Float>(t14519: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t4496: F, t1568: F, t836: F, t231: F, t2783: F) -> (F, F, F, F) {
    let t14520 = t14519 * t686;
    let t14522 = F::cast_from(0.19514881078765566038e-1_f64) * t2798 * t14520;
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    let t14535 = t1568 * t836;
    let t14537 = t2783 * t14535 * t231;
    (t14522, t14525, t14533, t14537)
}
