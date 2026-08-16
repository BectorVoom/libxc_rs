//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1332/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1332<F: Float>(t22829: F, t26028: F, t27932: F, t85776: F, t22890: F, t108516: F, t108524: F, t108537: F, t108539: F, t108554: F, t108559: F, t108562: F, t98141: F, t98148: F, t98161: F, t98165: F) -> F {
    let t114521 = t26028 * t22829;
    let t114525 = t27932 * t85776;
    let t114527 = t26028 * t22890;
    let t114536 = -F::cast_from(0.48018900292238105409e-1_f64) * t108516 + F::cast_from(0.6098400337114239387e-3_f64) * t108524 + F::cast_from(0.51448821741683684367e-2_f64) * t114521 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t108537 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t108539 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t114525 + F::cast_from(0.51448821741683684367e-2_f64) * t114527 - F::cast_from(0.85748036236139473943e-3_f64) * t108554 - F::cast_from(0.45732285992607719437e-3_f64) * t98141 + F::cast_from(0.32524801797942610064e-2_f64) * t98148 + F::cast_from(0.15246000842785598467e-4_f64) * t98161 - F::cast_from(0.34299214494455789577e-3_f64) * t108559 + F::cast_from(0.15246000842785598468e-3_f64) * t108562 - F::cast_from(0.13605355082800796533e0_f64) * t98165;
    t114536
}
