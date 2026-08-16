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
    let t5852 = F::cast_from(0.16068111111111111111e1_f64) * t5519;
    let t5859 = F::cast_from(0.46308888888888888888e0_f64) * t5557;
    let t5865 = F::cast_from(0.264729375e1_f64) * t5513 - F::cast_from(0.52945875e1_f64) * t5516 + F::cast_from(0.3529725e1_f64) * t5541 - t5852 + F::cast_from(0.20659e1_f64) * t5522 - F::cast_from(0.1549425e1_f64) * t5525 + F::cast_from(0.1549425e1_f64) * t5539 - F::cast_from(0.157790625e0_f64) * t5548 + F::cast_from(0.94674375e0_f64) * t5551 + F::cast_from(0.6311625e0_f64) * t5553 - t5859 + F::cast_from(0.104195e1_f64) * t5560 - F::cast_from(0.62517e0_f64) * t5563 - F::cast_from(0.62517e0_f64) * t5566 + F::cast_from(0.937755e0_f64) * t5570 + F::cast_from(0.312585e0_f64) * t5574;
    (t5846, t5852, t5859, t5865)
}
