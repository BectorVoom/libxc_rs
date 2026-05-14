//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1178/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1178<F: Float>(t22829: F, t26028: F, t27932: F, t85776: F, t22890: F, t108516: F, t108524: F, t108537: F, t108539: F, t108554: F, t108559: F, t108562: F, t98141: F, t98148: F, t98161: F, t98165: F) -> (F,) {
    let t114521 = t26028 * t22829;
    let t114525 = t27932 * t85776;
    let t114527 = t26028 * t22890;
    let t114536 = -0.48018900292238105409e-1 * t108516 + 0.6098400337114239387e-3 * t108524 + 0.51448821741683684367e-2 * t114521 + 7.0 / 48.0 * t108537 - 7.0 / 16.0 * t108539 + 3.0 / 16.0 * t114525 + 0.51448821741683684367e-2 * t114527 - 0.85748036236139473943e-3 * t108554 - 0.45732285992607719437e-3 * t98141 + 0.32524801797942610064e-2 * t98148 + 0.15246000842785598467e-4 * t98161 - 0.34299214494455789577e-3 * t108559 + 0.15246000842785598468e-3 * t108562 - 0.13605355082800796533e0 * t98165;
    (t114536,)
}
