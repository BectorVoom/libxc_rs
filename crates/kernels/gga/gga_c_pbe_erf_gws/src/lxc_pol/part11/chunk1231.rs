//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1231/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1231<F: Float>(t49463: F, t2168: F, t49305: F, t8599: F, t44814: F, t1133: F, t13290: F, t343: F, t28043: F, t3065: F, t858: F, t1105: F, t6241: F) -> (F, F, F, F, F, F) {
    let t49464 = param_a_c * t49463;
    let t49471 = F::new(3.0) / F::new(4.0) * t2168 * t8599 * t49305;
    let t49472 = F::new(7.0) / F::new(6.0) * t44814;
    let t49474 = t13290 * t1133 * t343;
    let t49478 = t28043 * t3065 * t858 * t49474 / F::new(12.0);
    let t49483 = t6241 * t1105;
    (t49464, t49471, t49472, t49474, t49478, t49483)
}
