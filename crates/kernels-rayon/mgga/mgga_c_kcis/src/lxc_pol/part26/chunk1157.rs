//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1157/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1157(t27520: f64, t29433: f64, t20961: f64, t585: f64, t1468: f64, t7296: f64, t27544: f64, t7299: f64, t2055: f64, t5748: f64, t2062: f64, t5752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29434 = t27520 * t29433;
    let t29436 = t20961 * t585;
    let t29438 = t1468 * t7296;
    let t29440 = t27544 * t7299;
    let t29442 = t5748 * t2055;
    let t29444 = t5752 * t2062;
    (t29434, t29436, t29438, t29440, t29442, t29444)
}
