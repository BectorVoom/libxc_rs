//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 266/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk266<F: Float>(t43: F, t50: F, t477: F, t479: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t806 = piecewise3::<F>(t44, F::new(0.0), F::new(2.0) / F::new(3.0) * t477);
    let t808 = piecewise3::<F>(t51, F::new(0.0), F::new(2.0) / F::new(3.0) * t479);
    let t810 = t806 / F::new(2.0) + t808 / F::new(2.0);
    t810
}
