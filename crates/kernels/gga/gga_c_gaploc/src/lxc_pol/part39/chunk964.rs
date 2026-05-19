//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 964/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk964<F: Float>(t2268: F, t32005: F, t894: F, t41596: F, t426: F, t535: F, t39671: F, t39674: F, t39677: F, t39679: F, t39681: F, t12837: F, t6305: F) -> (F, F, F, F, F, F, F, F) {
    let t42597 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t894 * t32005;
    let t42601 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t41596 * t426;
    let t42602 = F::cast_from(0.284550066356761496e-1_f64) * t39671;
    let t42603 = F::cast_from(0.142275033178380748e-1_f64) * t39674;
    let t42604 = F::cast_from(0.23712505529730124666e-2_f64) * t39677;
    let t42605 = F::cast_from(0.47425011059460249332e-2_f64) * t39679;
    let t42606 = F::cast_from(0.71137516589190373998e-2_f64) * t39681;
    let t42607 = t6305 * t12837;
    (t42597, t42601, t42602, t42603, t42604, t42605, t42606, t42607)
}
