//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 876/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk876(t5741: f64, t5894: f64, t589: f64, t1505: f64, t2016: f64, t1555: f64, t2069: f64, t4184: f64, t4189: f64, t4291: f64, t576: f64, t251: f64, t4301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5895 = t5741 + t5894;
    let t5896 = t5895 * t589;
    let t5897 = t2016 * t1505;
    let t5898 = t5897 * t1555;
    let t5899 = t4184 * t2069;
    let t5900 = t2069 * t1555;
    let t5902 = 2.0_f64 * t4189 * t5900;
    let t5903 = t576 * t4291;
    let t5904 = t251 * t4301;
    (t5895, t5896, t5897, t5898, t5899, t5900, t5902, t5903, t5904)
}
