//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 234/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk234(t1021: f64, t955: f64, t1011: f64, t1015: f64, t1018: f64, t11: f64, t139: f64, t175: f64, t197: f64, t198: f64, t201: f64, t972: f64) -> (f64, f64) {
    let t1022 = t1021 * t955;
    let t1031 = 0.619125e-2_f64 * t1011 * t198 - 0.123825e-1_f64 * t1015 * t1018 - 0.619125e-2_f64 * t197 * t1022 - 0.53062222222222222221e-1_f64 * t139 * t11 * t175 - 0.79593333333333333331e-1_f64 * t139 * t201 * t972;
    (t1022, t1031)
}
