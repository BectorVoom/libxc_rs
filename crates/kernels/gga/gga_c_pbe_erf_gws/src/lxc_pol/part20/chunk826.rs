//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 826/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk826<F: Float>(t1882: F, t2790: F, t2660: F, t2796: F, t1879: F, t2688: F, t5129: F, t587: F, t2555: F, t5125: F, t197: F, t5283: F) -> (F, F, F, F, F, F) {
    let t7617 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2790 * t1882;
    let t7619 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2660 * t2796;
    let t7623 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1879 * t2796;
    let t7663 = t5129 * t2688;
    let t7665 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t587 * t7663;
    let t7666 = t5125 * t2555;
    let t7668 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t587 * t7666;
    let t7669 = t5283 * t197;
    (t7617, t7619, t7623, t7665, t7668, t7669)
}
