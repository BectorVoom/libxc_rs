//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1477/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1477<F: Float>(t11404: F, t11409: F, t11444: F, t11450: F, t11517: F, t11521: F, t11551: F, t11554: F, t2943: F, t2944: F, t2968: F, t2970: F, t311: F, t41540: F, t41668: F, t41763: F, t41864: F, t41867: F, t41871: F, t41873: F, t41876: F, t41879: F, t41882: F, t41885: F, t41888: F, t41895: F, t41913: F, t41926: F, t953: F, t954: F) -> F {
    let t41930 = F::new(24.0) * t11404 * t11551 - F::new(24.0) * t11409 * t41668 * t954 - F::new(6.0) * t2943 * t41763 * t954 + t41864 + t41867 - t41871 - t41873 + t41876 + t41879 + t41882 + t41885 - t41888 + F::cast_from(0.3859675079686208416e3_f64) * t11404 * t11517 + F::cast_from(0.12865583598954028054e3_f64) * t2968 * t11444 * t2970 * t953 + F::cast_from(0.12414243100625616072e5_f64) * t11450 * t41895 * t2944 - F::cast_from(0.14035736694323150897e2_f64) * t11554 * t11521 - F::cast_from(0.19751673498613801407e-1_f64) * t41540 - F::new(0.310907e-1) * (t41913 + t41926) * t311;
    t41930
}
