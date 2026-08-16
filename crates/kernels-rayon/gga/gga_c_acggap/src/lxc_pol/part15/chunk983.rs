//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 983/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk983(t2268: f64, t30797: f64, t30543: f64, t8473: f64, t31419: f64, t4810: f64, t721: f64, t30673: f64, t1503: f64, t7329: f64, t1992: f64, t5616: f64, t7585: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34650 = t31419 * t4810 * t721;
    let t34655 = 0.34299214494455789578e-2_f64 * t30673;
    let t34659 = t7329 * t1503;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    (t34638, t34640, t34650, t34655, t34659, t34675)
}
