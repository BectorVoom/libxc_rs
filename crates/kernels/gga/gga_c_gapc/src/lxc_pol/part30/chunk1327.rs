//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1327/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1327<F: Float>(t224: F, t37496: F, t37642: F, t38080: F, t38515: F, t12574: F, t699: F, t12337: F, t12335: F, t12330: F, t12347: F, t12575: F) -> (F, F, F, F, F, F, F) {
    let t38518 = t224 * (t37496 + t37642 + t38080 + t38515);
    let t38520 = t699 * t12574;
    let t38525 = F::new(4.0) * t12337;
    let t38526 = F::new(4.0) * t12335;
    let t38527 = F::new(2.0) * t12330;
    let t38528 = F::new(4.0) * t12347;
    let t38530 = F::new(2.0) * t12575;
    (t38518, t38520, t38525, t38526, t38527, t38528, t38530)
}
