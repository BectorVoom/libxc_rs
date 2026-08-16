//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1316/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1316(t2213: f64, t238: f64, t4135: f64, t10607: f64, t801: f64, t10611: f64, t2176: f64, t242: f64, t4104: f64, t10547: f64, t779: f64, t1793: f64, t27037: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28794 = t238 * t2213 * t4135;
    let t28797 = t238 * t801 * t10607;
    let t28800 = t238 * t801 * t10611;
    let t28804 = t238 * t242 * t2176 * t4104;
    let t28808 = t238 * t242 * t779 * t10547;
    let t28813 = t27037 * t1793;
    (t28794, t28797, t28800, t28804, t28808, t28813)
}
