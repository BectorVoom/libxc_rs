//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1140/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1140(t39840: f64, t39841: f64, t6064: f64, t2608: f64, t37470: f64, t574: f64, t19839: f64, t20: f64, t3293: f64, t2124: f64, t24762: f64, t10810: f64, t1592: f64, t8160: f64) -> (f64, f64, f64, f64) {
    let t39843 = t39840 * t39841 * t6064;
    let t39846 = t574 * t37470 * t2608;
    let t39849 = t3293 * t19839 * t20;
    let t39851 = t39849 * t2124 * t24762;
    let t39854 = t1592 * t10810 * t8160;
    (t39843, t39846, t39851, t39854)
}
