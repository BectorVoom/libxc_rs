//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1184/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1184<F: Float>(t2268: F, t2440: F, t2756: F, t10135: F, t6313: F, t10132: F, t6305: F, t555: F, t7861: F, t888: F, t7863: F, t894: F) -> (F, F, F, F, F, F, F) {
    let t31786 = F::new(0.56910013271352299198e-1) * t2268 * t2440 * t2756;
    let t31788 = F::new(0.2276400530854091968e0) * t6313 * t10135;
    let t31790 = F::new(0.17073003981405689759e0) * t6305 * t10132;
    let t31792 = F::new(0.17073003981405689759e0) * t6305 * t10135;
    let t31793 = t555 * t7861;
    let t31796 = F::new(0.85365019907028448797e-1) * t2268 * t31793 * t888;
    let t31799 = F::new(0.28455006635676149599e-1) * t2268 * t894 * t7863;
    (t31786, t31788, t31790, t31792, t31793, t31796, t31799)
}
