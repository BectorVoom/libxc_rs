//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1158/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1158(t1262: f64, t1646: f64, t26961: f64, t3515: f64, t330: f64, t3622: f64, t1267: f64, t5310: f64, t1071: f64, t1268: f64, t4547: f64, t2844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28105 = t1646 * t1262;
    let t28106 = t26961 * t28105;
    let t28107 = t3515 * t28106;
    let t28110 = t3622 * t330;
    let t28111 = t1646 * t1267;
    let t28112 = t28110 * t28111;
    let t28113 = t5310 * t28112;
    let t28116 = t1268 * t1071;
    let t28117 = t28116 * t4547;
    let t28118 = t5310 * t28117;
    let t28123 = t1268 * t2844;
    (t28106, t28107, t28110, t28112, t28113, t28116, t28117, t28118, t28123)
}
