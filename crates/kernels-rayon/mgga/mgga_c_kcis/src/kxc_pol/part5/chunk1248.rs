//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1248/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1248(t20869: f64, t233: f64, t1489: f64, t4163: f64, t6284: f64, t4162: f64, t4160: f64, t1497: f64, t4171: f64, t4170: f64, t833: f64, t5653: f64) -> (f64, f64, f64, f64, f64) {
    let t20870 = t233 * t20869;
    let t20873 = t4163 * t6284 * t1489;
    let t20874 = t4162 * t20873;
    let t20875 = t4160 * t20874;
    let t20878 = t4171 * t6284 * t1497;
    let t20879 = t4170 * t20878;
    let t20880 = t4160 * t20879;
    let t20882 = t6284 * t833;
    let t20883 = t5653 * t20882;
    (t20870, t20875, t20880, t20882, t20883)
}
