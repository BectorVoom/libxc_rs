//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 638/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk638<F: Float>(t1985: F, t226: F, t163: F, t4577: F, t148: F, t147: F, t413: F) -> (F, F, F, F) {
    let t5952 = F::new(4.0) * t226 * t1985;
    let t5975 = t4577 * t163;
    let t5977 = F::new(0.31505407223141117834e-1) * t148 * t5975;
    let t5984 = t413 * t147;
    (t5952, t5975, t5977, t5984)
}
