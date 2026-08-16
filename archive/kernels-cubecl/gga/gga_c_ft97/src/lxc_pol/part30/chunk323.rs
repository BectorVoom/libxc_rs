//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 323/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk323<F: Float>(t4245: F, t4308: F, t312: F, t4239: F, t1263: F, t2253: F, t327: F, t703: F, t3691: F, t1091: F, t2923: F, t904: F) -> (F, F, F, F, F) {
    let t4309 = t4245 + t4308;
    let t4311 = t4239 * t312;
    let t4332 = t2253 * t1263;
    let t4334 = t703 * t327;
    let t4335 = t4334 * t3691;
    let t4339 = t2923 * t1091 * t904;
    (t4309, t4311, t4332, t4335, t4339)
}
