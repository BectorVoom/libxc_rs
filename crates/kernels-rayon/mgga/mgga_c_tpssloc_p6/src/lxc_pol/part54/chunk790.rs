//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 790/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk790(t6605: f64, t7500: f64, t1499: f64, t1898: f64, t249: f64, t1512: f64, t6614: f64, t1516: f64, t6621: f64, t6580: f64, t6587: f64, t6603: f64, t6618: f64, t7494: f64, t7498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7501 = t6605 * t7500;
    let t7503 = t1499 * t1898;
    let t7504 = t7503 * t249;
    let t7506 = t6614 * t1512;
    let t7508 = t6621 * t1516;
    let t7510 = -t6580 - t7494 / 48.0_f64 - t6587 - 0.12111826828242117256e-2_f64 * t7498 - t6603 - 0.20186378047070195427e-3_f64 * t7501 + t7504 / 1536.0_f64 - t7506 / 1536.0_f64 - t6618 - t7508 / 384.0_f64;
    (t7501, t7503, t7504, t7506, t7508, t7510)
}
