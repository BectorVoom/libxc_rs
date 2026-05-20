//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2947/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2947<F: Float>(t15421: F, t19318: F, t15101: F, t19321: F, t11299: F, t23565: F, t934: F, t2924: F, t4631: F, t6110: F, t11404: F, t11450: F, t11548: F, t15104: F, t15350: F, t15406: F, t1621: F, t1622: F, t19226: F, t19272: F, t19275: F, t19276: F, t19290: F, t23723: F, t23758: F, t23773: F, t2943: F, t2968: F, t4669: F, t4670: F, t6158: F, t6173: F, t63971: F, t953: F) -> (F, F, F, F, F) {
    let t78246 = F::new(18.0) * t15421 * t19318;
    let t78248 = F::new(12.0) * t15101 * t19321;
    let t78251 = F::new(24.0) * t11299 * t23565 * t934;
    let t78254 = F::new(18.0) * t2924 * t6110 * t4631;
    let t78279 = -F::new(6.0) * t15104 * t19272 + F::cast_from(0.96491876992155210402e2_f64) * t15406 * t19276 - t78246 + t78248 + t78251 - t78254 + F::new(18.0) * t2968 * t6158 * t4669 + F::cast_from(0.11579025239058625248e4_f64) * t11450 * t23723 * t953 - F::new(6.0) * t11548 * t23773 - F::new(6.0) * t2943 * t4670 * t6173 - F::new(6.0) * t2943 * t1622 * t19226 + F::cast_from(0.96491876992155210402e2_f64) * t11404 * t23758 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t63971 * t1621 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t19275 * t4669 + F::cast_from(0.10526802520742363173e2_f64) * t15350 * t19290;
    (t78246, t78248, t78251, t78254, t78279)
}
