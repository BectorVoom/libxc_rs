//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1122/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1122(t3308: f64, t6362: f64, t8030: f64, t2834: f64, t3344: f64, t3290: f64, t7301: f64, t3591: f64, t37972: f64, t10872: f64, t11736: f64, t1615: f64, t3320: f64, t783: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39545 = t6362 * t3308 * t8030;
    let t39548 = t2834 * t3344;
    let t39550 = t3290 * t7301;
    let t39552 = t37972 * t3591;
    let t39554 = t10872 * t11736;
    let t39558 = t783 * t978 * t1615 * t3320;
    (t39545, t39548, t39550, t39552, t39554, t39558)
}
