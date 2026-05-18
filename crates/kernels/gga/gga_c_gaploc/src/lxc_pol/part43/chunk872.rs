//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 872/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk872<F: Float>(t2268: F, t41596: F, t426: F, t535: F, t39671: F, t39674: F, t39677: F, t39679: F, t39681: F, t8195: F, t9189: F, t2854: F, t29975: F, t6320: F) -> (F, F, F, F, F, F, F, F) {
    let t42601 = F::new(0.28455006635676149599e-1) * t2268 * t535 * t41596 * t426;
    let t42602 = F::new(0.284550066356761496e-1) * t39671;
    let t42603 = F::new(0.142275033178380748e-1) * t39674;
    let t42604 = F::new(0.23712505529730124666e-2) * t39677;
    let t42605 = F::new(0.47425011059460249332e-2) * t39679;
    let t42606 = F::new(0.71137516589190373998e-2) * t39681;
    let t42629 = F::new(0.19918504644973304719e0) * t2268 * t9189 * t8195;
    let t42633 = F::new(0.17073003981405689759e1) * t2268 * t6320 * t2854 * t29975;
    (t42601, t42602, t42603, t42604, t42605, t42606, t42629, t42633)
}
