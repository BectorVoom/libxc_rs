//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 942/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk942(t22511: f64, t8307: f64, t8513: f64, t641: f64, t31: f64, t607: f64, t645: f64, t608: f64, t6504: f64, t8308: f64, t79: f64, t8306: f64) -> (f64, f64, f64, f64, f64) {
    let t113833 = t8513 * t8307 * t22511;
    let t113836 = t641 * t641;
    let t113864 = t645 * t31 * t607;
    let t113871 = t8308 * t608 * t6504;
    let t113875 = t8306 * t79;
    (t113833, t113836, t113864, t113871, t113875)
}
