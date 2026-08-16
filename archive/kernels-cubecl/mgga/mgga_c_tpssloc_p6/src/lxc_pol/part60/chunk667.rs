//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 667/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk667<F: Float>(t533: F, t8639: F, t1390: F, t1983: F, t2018: F, t3701: F, t2095: F, t1873: F, t7230: F, t2039: F) -> (F, F, F, F, F, F, F, F) {
    let t8640 = t533 * t8639;
    let t8641 = t8640 * t1390;
    let t8642 = t1983 * t8641;
    let t8643 = t3701 * t2018;
    let t8644 = t2095 * t8643;
    let t8645 = t1983 * t8644;
    let t8654 = F::cast_from(0.135e2_f64) * t7230 * t1873;
    let t8657 = t2039 * t1873;
    (t8640, t8641, t8642, t8643, t8644, t8645, t8654, t8657)
}
