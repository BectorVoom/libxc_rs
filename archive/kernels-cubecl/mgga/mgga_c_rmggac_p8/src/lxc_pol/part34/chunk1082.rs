//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1082/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1082<F: Float>(t1356: F, t289: F, t70441: F, t70443: F, t70479: F, t71832: F, t71850: F, t76137: F, t76492: F, t76495: F, t76497: F, t76499: F, t78104: F, t78514: F, t78518: F, t78522: F, t78526: F, t78528: F, t78529: F) -> F {
    let t78532 = F::cast_from(0.58171619854173713846e-5_f64) * t76137 - t71832 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t78104 + F::cast_from(0.29085809927086856923e-4_f64) * t70441 - F::cast_from(0.87257429781260570769e-4_f64) * t70443 + F::cast_from(0.76860658247009135557e-5_f64) * t76492 - t78514 - t78518 - t78522 - t78526 - t76495 - t76497 + t70479 - F::cast_from(0.35038612185802734376e-6_f64) * t76499 - t78528 + t71850 - F::cast_from(0.2363e1_f64) * t289 * t78529;
    t78532
}
