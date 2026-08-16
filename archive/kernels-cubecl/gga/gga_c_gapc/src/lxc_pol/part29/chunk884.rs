//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 884/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk884<F: Float>(t10236: F, t827: F, t1063: F, t10201: F, t10205: F, t10208: F, t10213: F, t10217: F, t10220: F, t10223: F, t10227: F, t10232: F, t10234: F) -> (F, F) {
    let t10237 = t10236 * t827;
    let t10238 = t10237 * t1063;
    let t10240 = -F::cast_from(0.74372214241464483348e-4_f64) * t10201 + F::cast_from(0.11742981196020707897e-4_f64) * t10205 + F::cast_from(0.58714905980103539485e-5_f64) * t10208 + F::cast_from(0.56366309740899397906e-3_f64) * t10213 - F::cast_from(0.33406432906439709826e-4_f64) * t10217 - F::cast_from(0.58714905980103539485e-5_f64) * t10220 - F::cast_from(0.342503618217270647e-5_f64) * t10223 - F::cast_from(0.342503618217270647e-5_f64) * t10227 - F::cast_from(0.20299047773010240345e-6_f64) * t10232 - F::cast_from(0.11742981196020707897e-4_f64) * t10234 - F::cast_from(0.58714905980103539485e-5_f64) * t10238;
    (t10237, t10240)
}
