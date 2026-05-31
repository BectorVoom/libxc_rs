//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 587/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk587<F: Float>(t2615: F, t593: F, t1010: F, t1648: F, t331: F, t589: F, t34: F, t591: F, t587: F, t1017: F, t597: F, t562: F) -> (F, F, F, F, F, F, F, F) {
    let t2617 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2615 * t593;
    let t2619 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1648 * t1010;
    let t2620 = t331 * t589;
    let t2621 = t591 * t34;
    let t2622 = t2620 * t2621;
    let t2624 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t2622;
    let t2625 = t597 * t1017;
    let t2626 = t2625 * t562;
    (t2617, t2619, t2620, t2621, t2622, t2624, t2625, t2626)
}
