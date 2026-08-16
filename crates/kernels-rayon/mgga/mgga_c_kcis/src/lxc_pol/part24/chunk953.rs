//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 953/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk953(t1180: f64, t6682: f64, t5043: f64, t5083: f64, t1175: f64, t6700: f64, t375: f64, t1200: f64, t6709: f64, t19655: f64, t358: f64, t387: f64) -> (f64, f64, f64, f64, f64) {
    let t20181 = t6682 * t1180;
    let t20183 = t5083 * t5043;
    let t20185 = t1175 * t6700;
    let t20186 = t375 * t20185;
    let t20188 = t6709 * t1200;
    let t20190 = t358 * t19655;
    let t20191 = t387 * t20190;
    (t20181, t20183, t20186, t20188, t20191)
}
