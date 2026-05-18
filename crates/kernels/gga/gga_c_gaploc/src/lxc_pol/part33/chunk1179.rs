//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1179/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1179<F: Float>(t31905: F, t10166: F, t6466: F, t9074: F, t25580: F, t4261: F, t4325: F, t6525: F, t7888: F, t10227: F, t1349: F, t1063: F, t2765: F, t30200: F) -> (F, F, F, F, F, F) {
    let t31906 = F::new(0.71137516589190373998e-2) * t31905;
    let t31908 = t9074 * t10166 * t6466;
    let t31909 = F::new(0.35568758294595186999e-2) * t31908;
    let t31911 = t9074 * t4261 * t25580;
    let t31912 = F::new(0.23712505529730124666e-2) * t31911;
    let t31914 = t6525 * t7888 * t4325;
    let t31915 = F::new(0.71137516589190373998e-2) * t31914;
    let t31918 = t1349 * t10227;
    let t31919 = F::new(0.31616674039640166222e-2) * t31918;
    let t31922 = F::new(0.17073003981405689759e0) * t1063 * t2765 * t30200;
    (t31906, t31909, t31912, t31915, t31919, t31922)
}
