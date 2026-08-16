//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 856/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk856<F: Float>(t2246: F, t3090: F, t3094: F, t3309: F, t840: F, t3075: F, t331: F, t2306: F, t3074: F) -> (F, F, F, F, F) {
    let t8641 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2246 * t3090;
    let t8643 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2246 * t3094;
    let t8646 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t3309;
    let t8652 = t3075 * t331;
    let t8653 = t2306 * t8652;
    let t8654 = t3074 * t8653;
    (t8641, t8643, t8646, t8652, t8654)
}
