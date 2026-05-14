//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 689/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk689<F: Float>(t1033: F, t5486: F, t169: F, t474: F, t619: F, t116: F, t5463: F, t1908: F, t198: F, t4043: F, t5059: F, t654: F, t667: F, t3028: F, t3163: F, t3153: F, t3157: F, t561: F) -> (F, F, F, F, F, F, F) {
    let t8743 = t5486 * t1033;
    let t8744 = t169 * t8743;
    let t8745 = t474 * t619;
    let t8746 = t8744 * t8745;
    let t8748 = t116 * t5463;
    let t8751 = t4043 * t198 * t1908 * t5059;
    let t8752 = t8748 * t8751;
    let t8754 = t654 * t667;
    let t8755 = t116 * t8754;
    let t8756 = t8755 * t8751;
    let t8758 = t3028 * t3163;
    let t8761 = t561 * t3153 * t3157;
    (t8746, t8751, t8752, t8754, t8756, t8758, t8761)
}
