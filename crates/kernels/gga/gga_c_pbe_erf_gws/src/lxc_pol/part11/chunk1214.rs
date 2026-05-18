//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1214/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1214<F: Float>(t12381: F, t823: F, t45660: F, t9016: F, t44296: F, t11787: F, t38264: F, t3116: F, t337: F, t3703: F, t3791: F, t6560: F) -> (F, F, F, F, F) {
    let t49239 = t823 * t12381;
    let t49245 = F::new(3.0) / F::new(4.0) * t9016 * t45660;
    let t49259 = F::new(7.0) / F::new(24.0) * t44296;
    let t49273 = t38264 * t11787 / F::new(8.0);
    let t49279 = F::new(3.0) / F::new(8.0) * t3116 * t6560 * t337 * t3791 * t3703;
    (t49239, t49245, t49259, t49273, t49279)
}
