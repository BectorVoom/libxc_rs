//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2947/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947(t15421: f64, t19318: f64, t15101: f64, t19321: f64, t11299: f64, t23565: f64, t934: f64, t2924: f64, t4631: f64, t6110: f64, t11404: f64, t11450: f64, t11548: f64, t15104: f64, t15350: f64, t15406: f64, t1621: f64, t1622: f64, t19226: f64, t19272: f64, t19275: f64, t19276: f64, t19290: f64, t23723: f64, t23758: f64, t23773: f64, t2943: f64, t2968: f64, t4669: f64, t4670: f64, t6158: f64, t6173: f64, t63971: f64, t953: f64) -> (f64, f64, f64, f64, f64) {
    let t78246 = 18.0_f64 * t15421 * t19318;
    let t78248 = 12.0_f64 * t15101 * t19321;
    let t78251 = 24.0_f64 * t11299 * t23565 * t934;
    let t78254 = 18.0_f64 * t2924 * t6110 * t4631;
    let t78279 = -6.0_f64 * t15104 * t19272 + 0.96491876992155210402e2_f64 * t15406 * t19276 - t78246 + t78248 + t78251 - t78254 + 18.0_f64 * t2968 * t6158 * t4669 + 0.11579025239058625248e4_f64 * t11450 * t23723 * t953 - 6.0_f64 * t11548 * t23773 - 6.0_f64 * t2943 * t4670 * t6173 - 6.0_f64 * t2943 * t1622 * t19226 + 0.96491876992155210402e2_f64 * t11404 * t23758 + 0.96491876992155210402e2_f64 * t2968 * t63971 * t1621 + 0.96491876992155210402e2_f64 * t2968 * t19275 * t4669 + 0.10526802520742363173e2_f64 * t15350 * t19290;
    (t78246, t78248, t78251, t78254, t78279)
}
