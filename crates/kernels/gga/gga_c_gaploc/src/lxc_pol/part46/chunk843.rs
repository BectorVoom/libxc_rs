//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 843/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk843<F: Float>(t1: F, t106: F, t12792: F, t192: F, t12865: F, t1580: F, t31828: F, t874: F, t1445: F, t597: F, t10151: F, t2293: F) -> (F, F, F, F, F) {
    let t41860 = t12792 * t1 * t106 * t192;
    let t41863 = t1580 * t12865;
    let t41865 = t31828 * t874;
    let t41867 = t597 * t1445 * t41865;
    let t41869 = t10151 * t2293;
    (t41860, t41863, t41865, t41867, t41869)
}
