//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 739/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk739(t1142: f64, t8081: f64, t2192: f64, t5345: f64, t1856: f64, t7773: f64, t5329: f64) -> (f64, f64, f64, f64) {
    let t8082 = t1142 * t8081;
    let t8083 = t5345 * t2192;
    let t8086 = t7773 * t1856;
    let t8087 = t5329 * t8086;
    (t8082, t8083, t8086, t8087)
}
