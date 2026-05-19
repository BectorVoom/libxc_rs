//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1279/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1279<F: Float>(t35463: F, t35466: F, t35471: F, t35475: F, t35478: F, t35480: F, t35482: F, t35485: F, t35489: F, t35493: F, t35495: F, t35500: F, t35503: F) -> F {
    let t37399 = -F::cast_from(0.72415202344614669852e-6_f64) * t35463 - F::cast_from(0.72415202344614669852e-6_f64) * t35466 - F::cast_from(0.1584082551288445903e-6_f64) * t35471 + F::cast_from(0.95044953077306754182e-5_f64) * t35475 + F::cast_from(0.95044953077306754182e-5_f64) * t35478 - F::cast_from(0.10122785552294833012e-4_f64) * t35480 + F::cast_from(0.84484402735383781496e-5_f64) * t35482 - F::cast_from(0.9110506997065349711e-4_f64) * t35485 - F::cast_from(0.10122785552294833012e-4_f64) * t35489 + F::cast_from(0.10122785552294833012e-4_f64) * t35493 + F::cast_from(0.50613927761474165061e-5_f64) * t35495 - F::cast_from(0.17379648562707520765e-3_f64) * t35500 + F::cast_from(0.10122785552294833012e-4_f64) * t35503;
    t37399
}
