//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1191/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1191(t12550: f64, t2201: f64, t3324: f64, t11760: f64, t11770: f64, t12459: f64, t3336: f64, t5095: f64, t2892: f64, t3319: f64, t3320: f64, t1592: f64, t30285: f64, t3308: f64) -> (f64, f64, f64, f64, f64) {
    let t43606 = t2201 * t12550 * t3324;
    let t43609 = t2201 * t11760 * t11770;
    let t43612 = t5095 * t3336 * t12459;
    let t43616 = t5095 * t3319 * t3320 * t2892;
    let t43619 = t1592 * t3308 * t30285;
    (t43606, t43609, t43612, t43616, t43619)
}
