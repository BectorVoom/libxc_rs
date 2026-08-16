//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 516/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk516(t1233: f64, t2115: f64, t2119: f64, t4037: f64, t4054: f64, t1248: f64, t2075: f64, t3979: f64, t2133: f64, t45: f64, t2141: f64, t4100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6035 = t2115 * t1233;
    let t6043 = t4037 * t2119;
    let t6059 = t4054 * t2119;
    let t6066 = t1248 * t3979 * t2075;
    let t6095 = t45 * t2133;
    let t6100 = t4100 * t2141;
    (t6035, t6043, t6059, t6066, t6095, t6100)
}
