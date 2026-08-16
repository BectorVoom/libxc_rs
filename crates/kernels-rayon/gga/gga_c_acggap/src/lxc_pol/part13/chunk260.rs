//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 260/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk260(t164: f64, t980: f64, t177: f64, t38: f64, t8: f64, t121: f64, t126: f64, t147: f64, t165: f64, t335: f64, t397: f64, t932: f64, t936: f64, t942: f64, t947: f64, t953: f64, t957: f64, t962: f64, t968: f64, t976: f64, t979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t981 = t980 * t164;
    let t983 = 0.21437009059034868486e-3_f64 * t981 * t177;
    let t985 = 1.0_f64 / t8 / t38;
    let t986 = t121 * t985;
    let t987 = t986 * t126;
    let t989 = 35.0_f64 / 432.0_f64 * t987 * t147;
    let t990 = -0.21437009059034868486e-3_f64 * t397 * t932 - 0.42874018118069736972e-3_f64 * t936 + 0.42874018118069736972e-3_f64 * t942 * t947 + 0.20007875121765877254e-2_f64 * t953 - 0.21437009059034868486e-3_f64 * t397 * t957 + t335 * t962 / 24.0_f64 + 0.42874018118069736972e-3_f64 * t165 * t968 + t976 - t979 + t983 + t989;
    (t983, t985, t986, t987, t989, t990)
}
