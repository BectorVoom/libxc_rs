//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1238/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1238<F: Float>(t45048: F, t13525: F, t37994: F, t11414: F, t37286: F, t45063: F, t3180: F, t45074: F, t45069: F, t11478: F, t2168: F, t3139: F, t3855: F) -> (F, F, F, F, F, F, F) {
    let t49577 = F::new(7.0) / F::new(24.0) * t45048;
    let t49579 = t37994 * t13525 / F::new(8.0);
    let t49581 = t37286 * t11414 / F::new(4.0);
    let t49585 = F::new(7.0) / F::new(24.0) * t45063;
    let t49588 = t45074 * t3180 / F::new(12.0);
    let t49594 = F::new(7.0) / F::new(12.0) * t45069;
    let t49607 = t2168 * t3139 * t11478 * t3855 / F::new(16.0);
    (t49577, t49579, t49581, t49585, t49588, t49594, t49607)
}
