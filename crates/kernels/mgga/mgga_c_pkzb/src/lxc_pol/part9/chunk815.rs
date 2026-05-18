//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 815/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk815<F: Float>(t5484: F, t5493: F, t5519: F, t5557: F, t5513: F, t5516: F, t5522: F, t5525: F, t5539: F, t5541: F, t5548: F, t5551: F, t5553: F, t5560: F, t5563: F, t5566: F, t5570: F, t5574: F) -> (F, F, F, F) {
    let t5846 = t5484 * t5493;
    let t5852 = F::new(0.16068111111111111111e1) * t5519;
    let t5859 = F::new(0.46308888888888888888e0) * t5557;
    let t5865 = F::new(0.264729375e1) * t5513 - F::new(0.52945875e1) * t5516 + F::new(0.3529725e1) * t5541 - t5852 + F::new(0.20659e1) * t5522 - F::new(0.1549425e1) * t5525 + F::new(0.1549425e1) * t5539 - F::new(0.157790625e0) * t5548 + F::new(0.94674375e0) * t5551 + F::new(0.6311625e0) * t5553 - t5859 + F::new(0.104195e1) * t5560 - F::new(0.62517e0) * t5563 - F::new(0.62517e0) * t5566 + F::new(0.937755e0) * t5570 + F::new(0.312585e0) * t5574;
    (t5846, t5852, t5859, t5865)
}
