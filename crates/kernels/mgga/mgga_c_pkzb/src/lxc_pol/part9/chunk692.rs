//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 692/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk692<F: Float>(t3236: F, t824: F, t758: F, t179: F, t3026: F, t932: F, t1238: F, t2377: F, t2380: F, t2404: F, t2408: F, t3206: F, t3209: F, t3214: F, t3217: F, t3225: F, t3230: F, t3235: F, t404: F, t923: F, t934: F) -> (F, F) {
    let t3237 = t3236 * t824;
    let t3238 = t758 * t3237;
    let t3242 = t179 * t932 * t3026;
    let t3245 = -F::new(0.21437009059034868486e-3) * t3206 * t3209 - F::new(0.11433071498151929859e-2) * t3214 * t923 - F::new(0.7622047665434619906e-3) * t3217 + F::new(0.22866142996303859718e-2) * t1238 * t934 + F::new(0.14291339372689912324e-3) * t2377 - t2404 - F::new(0.28582678745379824648e-3) * t2408 - F::new(0.42874018118069736972e-3) * t2380 * t3225 - F::new(0.28582678745379824648e-3) * t3230 + F::new(0.12862205435420921092e-2) * t3235 * t3238 - F::new(0.42874018118069736972e-3) * t404 * t3242;
    (t3237, t3245)
}
