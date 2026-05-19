//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1431/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1431<F: Float>(t33261: F, t36520: F, t36521: F, t36522: F, t36523: F, t36524: F, t36526: F, t36527: F, t36528: F, t36529: F, t36530: F, t33353: F, t33375: F, t33377: F, t33380: F, t36559: F, t36560: F, t36561: F, t36562: F, t36563: F, t36564: F, t36568: F) -> (F, F) {
    let t38716 = -t36520 + t36521 + t36522 - t36523 - t36524 + F::cast_from(0.97817934710145362364e-6_f64) * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    let t38726 = F::cast_from(0.90579542097823505428e-7_f64) * t33353 + t36559 + t36560 - t36561 + t36562 + t36563 + t36564 - F::cast_from(0.67632724766374884054e-5_f64) * t33375 - F::cast_from(0.54347725258694103258e-6_f64) * t33377 - F::cast_from(0.18115908419564701086e-6_f64) * t33380 - t36568;
    (t38716, t38726)
}
