//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 568/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk568<F: Float>(t1470: F, t3382: F, t1562: F, t3379: F, t1466: F, t1545: F, t3431: F, t1524: F, t322: F, t1095: F, t398: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t4689 = F::new(0.85748036236139473944e-3) * t3382 * t1470;
    let t4699 = F::new(0.17149607247227894789e-2) * t3379 * t1562;
    let t4705 = F::new(0.85748036236139473944e-3) * t3382 * t1466;
    let t4716 = t3431 * t1545;
    let t4718 = t1524 * t322;
    let t4720 = t398 * t1095 * t4718;
    let t4722 = F::new(0.85748036236139473944e-3) * t384 * t4720;
    (t4689, t4699, t4705, t4716, t4718, t4720, t4722)
}
