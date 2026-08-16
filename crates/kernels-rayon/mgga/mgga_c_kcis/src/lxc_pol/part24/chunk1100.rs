//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1100/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1100(t28931: f64, t28964: f64, t28995: f64, t29022: f64, t393: f64, t1820: f64, t27987: f64, t26871: f64, t6638: f64, t6735: f64, t7740: f64, t19826: f64, t2189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29024 = t28931 + t28964 + t28995 + t29022;
    let t29025 = t29024 * t393;
    let t29027 = 2.0_f64 * t27987 * t1820;
    let t29029 = 2.0_f64 * t26871 * t6638;
    let t29030 = t7740 * t6735;
    let t29031 = t19826 * t2189;
    (t29024, t29025, t29027, t29029, t29030, t29031)
}
