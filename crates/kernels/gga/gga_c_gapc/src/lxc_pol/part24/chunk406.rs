//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 406/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk406<F: Float>(t231: F, t4: F, t1220: F, t283: F, t482: F, t132: F, t762: F, t737: F, t88: F, t256: F, t62: F, t748: F) -> (F, F, F, F, F, F) {
    let t2040 = t231 * t4;
    let t2042 = F::new(0.10843580882781524214e-1) * t2040 * t1220;
    let t2043 = t482 * t283;
    let t2046 = t132 * t762;
    let t2053 = t88 * t737;
    let t2056 = t256 * t256;
    let t2057 = F::new(1.0) / t2056;
    let t2058 = t62 * t2057;
    let t2059 = t748 * t748;
    (t2042, t2043, t2046, t2053, t2058, t2059)
}
