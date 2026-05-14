//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 956/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk956<F: Float>(t24021: F, t256: F, t23682: F, t23801: F, t243: F, t2491: F, t2516: F, t23685: F, t2519: F, t24565: F, t2661: F, t329: F, t23548: F, t7856: F, t7298: F, t896: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24733 = t256 * t24021;
    let t24776 = 0.17757530864197530864e0 * t23682;
    let t24795 = t256 * t23801;
    let t24804 = t243 / t2516 / t2491;
    let t24863 = 0.5356037037037037037e1 * t23682;
    let t24864 = 0.16979925925925925926e1 * t23685;
    let t24879 = t2516 * t2516;
    let t24881 = t243 / t24879;
    let t24882 = t2519 * t2519;
    let t24883 = 1.0 / t24882;
    let t24989 = t2661 * t24565;
    let t24995 = t329 * t24565;
    let t25001 = t7856 * t23548;
    let t25085 = t896 * t7298;
    (t24733, t24776, t24795, t24804, t24863, t24864, t24881, t24883, t24989, t24995, t25001, t25085)
}
