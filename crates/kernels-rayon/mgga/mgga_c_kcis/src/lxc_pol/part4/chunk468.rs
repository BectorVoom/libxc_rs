//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 468/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk468(t1911: f64, t482: f64, t1349: f64, t1352: f64, t1891: f64, t1898: f64, t1901: f64, t1904: f64) -> (f64, f64) {
    let t1912 = t1911 * t482;
    let t1919 = 0.258925e1_f64 * t1898 - t1349 - 0.301925e0_f64 * t1891 + 0.16504875e0_f64 * t1901 - t1352 - 0.82785e-1_f64 * t1904;
    (t1912, t1919)
}
