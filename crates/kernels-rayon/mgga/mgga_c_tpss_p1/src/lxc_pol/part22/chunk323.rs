//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 323/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk323(t1042: f64, t1043: f64, t1024: f64, t1011: f64, t1017: f64, t417: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1044 = t1042 * t1043;
    let t1046 = 1.0_f64 * t1024 * t1044;
    let t1047 = 0.17123333333333333333e-1_f64 * t1011;
    let t1049 = -t1047 + 0.17123333333333333333e-1_f64 * t1017;
    let t1052 = t417 * t417;
    let t1053 = 1.0_f64 / t1052;
    (t1044, t1046, t1047, t1049, t1052, t1053)
}
