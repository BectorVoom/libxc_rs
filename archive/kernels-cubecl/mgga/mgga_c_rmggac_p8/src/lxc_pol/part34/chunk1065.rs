//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1065/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1065<F: Float>(t75803: F, t1627: F, t3204: F, t71633: F, t71639: F, t75771: F, t75774: F, t75780: F, t75792: F, t75797: F, t78287: F, t78288: F, t78290: F, t78295: F, t78298: F, t78299: F, t78301: F, t78303: F, t903: F) -> F {
    let t78304 = F::cast_from(0.2627895913935205078e-5_f64) * t75803;
    let t78305 = F::cast_from(0.18637685463734316849e-1_f64) * t75771 - F::cast_from(0.46594213659335792122e-1_f64) * t75774 - F::cast_from(0.93188427318671584245e-2_f64) * t75780 - t78287 + t78288 - t71633 + F::cast_from(0.87596530464506835935e-6_f64) * t75792 + t78290 + t71639 + F::cast_from(0.17961362552795712846e0_f64) * t903 * t3204 * t1627 - t78295 - t78298 - t78299 - t78301 - F::cast_from(0.17519306092901367187e-5_f64) * t75797 + t78303 - t78304;
    t78305
}
