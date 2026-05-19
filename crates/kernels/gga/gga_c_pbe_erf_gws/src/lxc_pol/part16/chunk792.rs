//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 792/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk792<F: Float>(t163: F, t169: F, t2019: F, t299: F, t4577: F, t148: F, t1602: F, t547: F, t1964: F, t536: F, t147: F, t413: F) -> (F, F, F, F, F) {
    let t5973 = t169 * t299 * t2019 * t163;
    let t5975 = t4577 * t163;
    let t5977 = F::cast_from(0.31505407223141117834e-1_f64) * t148 * t5975;
    let t5980 = t1602 * t547;
    let t5982 = t536 * t1964;
    let t5984 = t413 * t147;
    (t5973, t5977, t5980, t5982, t5984)
}
