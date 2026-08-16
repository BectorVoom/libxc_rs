//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1148/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1148(t2196: f64, t25177: f64, t3308: f64, t3588: f64, t37932: f64, t10894: f64, t8243: f64, t10810: f64, t2184: f64, t7629: f64, t7625: f64, t26314: f64, t37755: f64, t39841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39975 = t2196 * t3308 * t25177;
    let t39977 = t37932 * t3588;
    let t39979 = t10894 * t8243;
    let t39982 = t2184 * t10810 * t7629;
    let t39984 = t10894 * t7625;
    let t39987 = t37755 * t39841 * t26314;
    (t39975, t39977, t39979, t39982, t39984, t39987)
}
