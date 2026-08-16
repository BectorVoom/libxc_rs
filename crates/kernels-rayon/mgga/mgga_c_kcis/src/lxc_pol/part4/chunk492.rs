//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 492/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk492(t1506: f64, t2069: f64, t1557: f64, t1891: f64, t1566: f64, t1569: f64, t1898: f64, t1901: f64, t1904: f64, t1572: f64) -> (f64, f64, f64, f64) {
    let t2070 = t1506 * t2069;
    let t2072 = -t1557 - 0.17123333333333333333e-1_f64 * t1891;
    let t2079 = 0.3529725e1_f64 * t1898 - t1566 - 0.516475e0_f64 * t1891 + 0.6311625e0_f64 * t1901 - t1569 - 0.104195e0_f64 * t1904;
    let t2080 = t2079 * t1572;
    (t2070, t2072, t2079, t2080)
}
