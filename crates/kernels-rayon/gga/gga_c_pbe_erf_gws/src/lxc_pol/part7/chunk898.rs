//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 898/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk898(t1697: f64, t16986: f64, t11: f64, t625: f64, t4373: f64, t5037: f64, t16973: f64, t5063: f64, t1691: f64, t16960: f64, t16962: f64, t16964: f64, t16966: f64, t16968: f64, t16976: f64, t16981: f64, t16985: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16987 = t1697 * t16986;
    let t16989 = t11 * t625 * t16987;
    let t16991 = t5037 * t4373;
    let t16993 = t11 * t625 * t16991;
    let t16995 = t5063 * t16973;
    let t16997 = t11 * t1691 * t16995;
    let t16999 = -0.78365432098765432099e-2_f64 * t16960 + 0.50377777777777777778e-2_f64 * t16962 + 0.33585185185185185186e-2_f64 * t16964 - 0.25188888888888888889e-2_f64 * t16966 - 0.27987654320987654323e-2_f64 * t16968 + 0.55975308641975308645e-2_f64 * t16976 + 0.18891666666666666667e-2_f64 * t16981 - t16985 - 0.11335e-1_f64 * t16989 - 0.15113333333333333333e-1_f64 * t16993 + 0.45340000000000000001e-1_f64 * t16997;
    (t16987, t16989, t16991, t16993, t16995, t16997, t16999)
}
