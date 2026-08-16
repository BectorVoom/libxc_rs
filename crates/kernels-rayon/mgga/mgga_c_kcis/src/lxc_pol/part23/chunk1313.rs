//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1313/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1313(t12345: f64, t1555: f64, t28576: f64, t4189: f64, t4310: f64, t8207: f64, t2069: f64, t94197: f64, t4479: f64, t8236: f64, t28558: f64, t1505: f64, t28556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99713 = 12.0_f64 * t12345 * t28576 * t1555;
    let t99716 = 2.0_f64 * t4189 * t8207 * t4310;
    let t99717 = t94197 * t2069;
    let t99718 = t8236 * t4479;
    let t99723 = t28558 * t4310;
    let t99724 = t28556 * t1505;
    (t99713, t99716, t99717, t99718, t99723, t99724)
}
