//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 834/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk834(t13917: f64, t8318: f64, t1580: f64, t6459: f64, t6473: f64, t1610: f64, t8432: f64, t1149: f64, t7724: f64, t2527: f64, t7715: f64, t6666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27920 = t13917 * t8318;
    let t27921 = t1580 * t27920;
    let t27925 = t6459 * t6473;
    let t28036 = t8432 * t1610;
    let t28152 = t7724 * t1149;
    let t28208 = t7715 * t2527;
    let t28209 = t6666 * t28208;
    (t27921, t27925, t28036, t28152, t28208, t28209)
}
