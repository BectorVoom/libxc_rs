//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1125/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1125<F: Float>(t92997: F, t92999: F, t93007: F, t93012: F, t93020: F, t7406: F, t9288: F, t7064: F, t2453: F, t26496: F, t26555: F, t40270: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95674 = F::new(0.15117061203111996147e0) * t92997;
    let t95675 = F::new(0.51384669507166276316e-2) * t92999;
    let t95678 = F::new(0.80328230880474379779e-6) * t93007;
    let t95680 = F::new(0.45178982497454656792e-6) * t93012;
    let t95684 = F::new(0.28900264064772933812e-2) * t93020;
    let t95730 = t7406 * t9288;
    let t95732 = F::new(0.39982213492741449076e-1) * t7064 * t95730;
    let t95773 = t2453 * t26496;
    let t95807 = F::new(0.96373646535613327356e-3) * t40270 * t26555;
    (t95674, t95675, t95678, t95680, t95684, t95730, t95732, t95773, t95807)
}
