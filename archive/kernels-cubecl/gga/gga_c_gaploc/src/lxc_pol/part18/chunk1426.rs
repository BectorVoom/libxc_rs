//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1426/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1426<F: Float>(t20168: F, t31540: F, t20158: F, t31735: F, t20172: F, t2854: F, t590: F, t6519: F, t2875: F, t544: F, t6514: F, t1367: F, t20901: F) -> (F, F, F, F) {
    let t35172 = F::cast_from(0.51123901271894332902e1_f64) * t20168 * t31540;
    let t35174 = F::cast_from(0.2044956050875773316e1_f64) * t20158 * t31735;
    let t35178 = F::cast_from(0.30674340763136599742e1_f64) * t20172 * t2854 * t6519 * t590;
    let t35180 = t544 * t6514 * t2875;
    let t35183 = F::cast_from(0.55611873258433997041e0_f64) * t35180 * t20901 * t1367;
    (t35172, t35174, t35178, t35183)
}
