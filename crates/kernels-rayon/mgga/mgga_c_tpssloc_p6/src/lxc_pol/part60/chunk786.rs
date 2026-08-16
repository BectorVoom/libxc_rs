//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 786/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk786(t5612: f64, t815: f64, t6605: f64, t1898: f64, t5575: f64, t249: f64, t5628: f64, t6621: f64, t5619: f64, t6614: f64, t23048: f64, t5587: f64) -> (f64, f64, f64, f64, f64) {
    let t28356 = t815 * t5612;
    let t28357 = t6605 * t28356;
    let t28359 = t5575 * t1898;
    let t28360 = t28359 * t249;
    let t28362 = t6621 * t5628;
    let t28364 = t6614 * t5619;
    let t28366 = t23048 * t5587;
    (t28357, t28360, t28362, t28364, t28366)
}
