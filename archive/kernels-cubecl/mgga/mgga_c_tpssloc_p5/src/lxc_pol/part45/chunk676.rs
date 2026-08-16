//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 676/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk676<F: Float>(t533: F, t8639: F, t1390: F, t1983: F, t2018: F, t3701: F, t2095: F, t1869: F, t1976: F, t2036: F, t2040: F, t2075: F, t2096: F, t510: F, t574: F, t6517: F, t652: F, t8329: F, t8450: F, t8519: F, t8522: F, t8528: F, t8529: F, t8535: F, t8596: F, t8604: F, t8608: F) -> (F, F, F, F, F) {
    let t8640 = t533 * t8639;
    let t8641 = t8640 * t1390;
    let t8642 = t1983 * t8641;
    let t8643 = t3701 * t2018;
    let t8644 = t2095 * t8643;
    let t8645 = t1983 * t8644;
    let t8646 = -t1869 * t2075 - t1976 * t2036 - F::cast_from(2.0_f64) * t2040 * t6517 + t2096 * t8450 - t510 * t8519 + t574 * t8604 - F::cast_from(2.0_f64) * t652 * t8529 - t8329 - t8522 - t8528 - t8535 - t8596 + t8608 + t8642 - t8645;
    (t8640, t8641, t8643, t8644, t8646)
}
