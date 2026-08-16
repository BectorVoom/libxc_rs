//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1076/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1076(t1267: f64, t1646: f64, t28110: f64, t5310: f64, t1071: f64, t1268: f64, t4547: f64, t2844: f64, t5302: f64, t1262: f64, t1856: f64, t26996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28111 = t1646 * t1267;
    let t28112 = t28110 * t28111;
    let t28113 = t5310 * t28112;
    let t28116 = t1268 * t1071;
    let t28117 = t28116 * t4547;
    let t28118 = t5310 * t28117;
    let t28123 = t1268 * t2844;
    let t28124 = t28123 * t4547;
    let t28125 = t5302 * t28124;
    let t28130 = t1856 * t1262;
    let t28131 = t26996 * t28130;
    (t28111, t28112, t28113, t28116, t28117, t28118, t28123, t28124, t28125, t28130, t28131)
}
