//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1211/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1211(t1656: f64, t5918: f64, t5740: f64, t1838: f64, t4516: f64, t18967: f64, t19535: f64, t3255: f64) -> (f64, f64, f64, f64) {
    let t20178 = t5918 * t1656;
    let t20179 = t5740 * t20178;
    let t20182 = t1838 * t4516;
    let t20183 = t5740 * t20182;
    let t20187 = t18967 * t19535;
    let t20190 = t3255 * t1838;
    (t20179, t20183, t20187, t20190)
}
