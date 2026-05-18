//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 946/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk946<F: Float>(t33452: F, t668: F, t1434: F, t2399: F, t7520: F, t2248: F, t322: F, t7511: F, t7516: F, t2253: F, t6108: F, t33296: F) -> (F, F, F, F, F, F, F, F) {
    let t141357 = t33452 * t668;
    let t141363 = t1434 * t2399 * t7520;
    let t141364 = F::new(4.0) / F::new(9.0) * t141363;
    let t141365 = t2248 * t322;
    let t141367 = t7511 * t141365 * t7516;
    let t141368 = F::new(10.0) / F::new(9.0) * t141367;
    let t141369 = t6108 * t2253;
    let t141370 = t141369 * t33296;
    (t141357, t141363, t141364, t141365, t141367, t141368, t141369, t141370)
}
