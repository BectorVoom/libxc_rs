//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 865/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk865(t1180: f64, t12992: f64, t311: f64, t313: f64, t3841: f64, t12974: f64, t12983: f64, t3661: f64, t26: f64, t1186: f64, t12868: f64, t306: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12995 = t1180 * t12992;
    let t12998 = t311 * t3841 * t313;
    let t12999 = 0.36514074074074074075e0_f64 * t12998;
    let t13000 = 0.93011851851851851854e0_f64 * t12974;
    let t13001 = t3661 * t12983;
    let t13002 = t26 * t13001;
    let t13004 = t1186 * t12868;
    let t13005 = t26 * t13004;
    let t13009 = 1.0_f64 / t306 / t315 / 4.0_f64;
    (t12995, t12998, t12999, t13000, t13002, t13005, t13009)
}
