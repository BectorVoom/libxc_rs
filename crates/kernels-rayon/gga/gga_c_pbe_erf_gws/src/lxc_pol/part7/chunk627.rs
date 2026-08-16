//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 627/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk627(t1815: f64, t4882: f64, t639: f64, t1406: f64, t572: f64, t418: f64, t1821: f64, t1820: f64, t1866: f64, t1827: f64, t587: f64, t1724: f64, t626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4883 = t1815 * t4882;
    let t4885 = 8.0_f64 / 15.0_f64 * t639 * t4883;
    let t4886 = t1406 * t572;
    let t4887 = t4886 * t418;
    let t4888 = t1821 * t4887;
    let t4890 = 8.0_f64 / 15.0_f64 * t1820 * t4888;
    let t4891 = t1866 * t572;
    let t4892 = t4891 * t418;
    let t4893 = t1827 * t4892;
    let t4895 = 4.0_f64 / 15.0_f64 * t587 * t4893;
    let t4896 = t1724 * t626;
    (t4883, t4885, t4886, t4887, t4888, t4890, t4891, t4892, t4893, t4895, t4896)
}
