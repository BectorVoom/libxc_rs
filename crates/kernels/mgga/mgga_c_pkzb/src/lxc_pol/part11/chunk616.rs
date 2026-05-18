//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 616/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk616<F: Float>(t3410: F, t51: F, t1721: F, t592: F, t1020: F, t2600: F, t179: F, t2610: F, t2608: F, t2615: F, t2617: F, t2621: F) -> (F, F, F, F, F, F, F, F) {
    let t3411 = t51 * t3410;
    let t3413 = t592 * t3411 * t1721;
    let t3417 = t2600 * t1020;
    let t3418 = t179 * t3417;
    let t3421 = F::new(0.11696447245269292414e1) * t2610;
    let t3422 = F::new(2.0) * t2608;
    let t3423 = F::new(8.0) * t2615;
    let t3424 = F::new(8.0) * t2617;
    let t3425 = F::new(0.36622894612013090108e-3) * t2621;
    (t3411, t3413, t3418, t3421, t3422, t3423, t3424, t3425)
}
