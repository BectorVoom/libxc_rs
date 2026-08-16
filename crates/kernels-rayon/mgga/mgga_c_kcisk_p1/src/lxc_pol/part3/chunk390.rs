//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 390/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk390(t801: f64, t798: f64, t1932: f64, t1938: f64, t1942: f64, t1946: f64, t1951: f64, t1955: f64) -> (f64, f64, f64, f64) {
    let t2040 = t801 * t801;
    let t2041 = 1.0_f64 / t2040;
    let t2042 = t798 * t2041;
    let t2049 = 0.9375e-1_f64 * t1932 - 0.9375e-1_f64 * t1938 + 0.625e-1_f64 * t1942 - 0.101171875e-1_f64 * t1946 + 0.101171875e-1_f64 * t1951 - 0.13489583333333333333e-1_f64 * t1955;
    (t2040, t2041, t2042, t2049)
}
