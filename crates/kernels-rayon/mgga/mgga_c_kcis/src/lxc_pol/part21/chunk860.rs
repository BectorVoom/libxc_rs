//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 860/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk860(t13250: f64, t3210: f64, t13172: f64, t4793: f64, t9425: f64, t5042: f64, t922: f64, t3202: f64, t3200: f64, t1767: f64, t3219: f64, t3218: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13251 = t3210 * t13250;
    let t13252 = t13172 * t13251;
    let t13254 = t9425 * t4793;
    let t13256 = t5042 * t922;
    let t13257 = t3202 * t13256;
    let t13258 = t3200 * t13257;
    let t13260 = t1767 * t3219;
    let t13261 = t3218 * t13260;
    (t13252, t13254, t13256, t13258, t13260, t13261)
}
