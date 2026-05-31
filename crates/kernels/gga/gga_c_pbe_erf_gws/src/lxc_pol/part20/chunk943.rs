//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 943/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk943<F: Float>(t3456: F, t579: F, t1033: F, t2749: F, t3513: F, t7011: F, t4913: F, t2607: F, t2722: F, t1621: F, t1620: F, t2603: F, t2612: F) -> (F, F, F, F, F, F) {
    let t10616 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t579 * t3456;
    let t10617 = t1033 * t2749;
    let t10618 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10617;
    let t10620 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7011 * t3513;
    let t10622 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4913 * t3513;
    let t10623 = t2607 * t2722;
    let t10624 = t1621 * t10623;
    let t10626 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1620 * t10624;
    let t10628 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2612 * t2603;
    (t10616, t10618, t10620, t10622, t10626, t10628)
}
