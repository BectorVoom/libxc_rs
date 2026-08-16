//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 881/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk881(t3587: f64, t425: f64, t1364: f64, t3564: f64, t3565: f64, t3619: f64, t12825: f64, t458: f64, t12829: f64, t459: f64, t12830: f64, t1175: f64, t1354: f64, t3593: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13211 = t425 * t3587;
    let t13212 = t13211 * t1364;
    let t13213 = t3564 * t13212;
    let t13216 = t3565 * t3619;
    let t13217 = t3564 * t13216;
    let t13220 = t12825 * t458;
    let t13221 = t459 * t12829;
    let t13223 = t13220 * t13221 * t12830;
    let t13227 = t1354 * t1175 * t3593;
    (t13212, t13213, t13216, t13217, t13223, t13227)
}
