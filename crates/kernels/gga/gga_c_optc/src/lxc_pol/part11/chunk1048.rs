//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1048/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1048<F: Float>(t26264: F, t373: F, t26261: F, t56: F, t8950: F, t2848: F, t136: F, t8425: F, t22502: F, t370: F, t376: F, t2933: F, t2972: F, t393: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26265 = F::cast_from(0.13388493827160493828e1_f64) * t26264;
    let t26266 = F::powf(t373, -F::new(0.25e1));
    let t26313 = F::new(280.0) / F::new(81.0) * t26261;
    let t26334 = t56 * t8950;
    let t26335 = t2848 * t2848;
    let t26336 = F::new(1.0) / t26335;
    let t26374 = t136 * t8425;
    let t26424 = F::new(1.0) / t376 / t22502 / t370 / F::new(96.0);
    let t26496 = F::cast_from(0.31310740740740740741e1_f64) * t26261;
    let t26497 = F::cast_from(0.13490888888888888889e1_f64) * t26264;
    let t26593 = t393 / t2972 / t2933;
    (t26265, t26266, t26313, t26334, t26336, t26374, t26424, t26496, t26497, t26593)
}
