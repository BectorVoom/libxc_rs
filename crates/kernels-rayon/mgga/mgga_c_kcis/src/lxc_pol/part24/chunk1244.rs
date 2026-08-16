//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1244/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1244(t1092: f64, t1773: f64, t26760: f64, t43053: f64, t1121: f64, t6704: f64, t28093: f64, t28190: f64, t1133: f64, t67493: f64, t7718: f64, t69560: f64) -> (f64, f64, f64, f64, f64) {
    let t100297 = t1092 * t26760 * t43053 * t1773;
    let t100301 = t1092 * t26760 * t6704 * t1121;
    let t100303 = t28190 * t28093;
    let t100307 = t1092 * t7718 * t67493 * t1133;
    let t100312 = t1092 * t26760 * t69560 * t1133;
    (t100297, t100301, t100303, t100307, t100312)
}
