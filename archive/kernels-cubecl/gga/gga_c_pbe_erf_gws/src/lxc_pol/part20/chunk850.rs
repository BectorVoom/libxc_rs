//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 850/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk850<F: Float>(t230: F, t2962: F, t1049: F, t678: F, t837: F, t991: F, t551: F, t553: F, t1052: F, t163: F, t169: F, t784: F) -> (F, F, F, F) {
    let t8439 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2962 * t230;
    let t8440 = t1049 * t678;
    let t8465 = t837 * t991;
    let t8467 = t8465 * t551 * t553;
    let t8471 = t169 * t784 * t1052 * t163;
    (t8439, t8440, t8467, t8471)
}
