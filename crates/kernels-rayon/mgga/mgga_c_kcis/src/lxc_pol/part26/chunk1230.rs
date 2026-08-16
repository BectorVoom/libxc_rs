//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1230/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1230(t1377: f64, t1593: f64, t1444: f64, t3717: f64, t52613: f64, t7908: f64, t7910: f64, t1598: f64, t37602: f64, t11418: f64, t1386: f64, t1466: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94246 = t1593 * t1377;
    let t94274 = t3717 * t1444;
    let t94287 = t7908 * t52613 * t7910;
    let t94390 = t37602 * t1598;
    let t94408 = t1386 * t11418;
    let t94424 = t1466 * t491;
    (t94246, t94274, t94287, t94390, t94408, t94424)
}
