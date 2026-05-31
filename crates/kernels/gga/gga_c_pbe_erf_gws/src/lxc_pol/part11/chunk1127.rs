//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1127/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1127<F: Float>(t47377: F, t5002: F, t11: F, t625: F, t1697: F, t47733: F, t16972: F, t16970: F, t10523: F, t3354: F, t1714: F, t24980: F, t25: F, t32405: F, t40954: F, t40956: F, t40958: F, t40960: F, t41939: F, t41941: F) -> (F, F, F, F, F, F, F, F) {
    let t47949 = t5002 * t47377;
    let t47951 = t11 * t625 * t47949;
    let t47953 = t1697 * t47733;
    let t47955 = t11 * t625 * t47953;
    let t47957 = t16972 * t47377;
    let t47959 = t11 * t16970 * t47957;
    let t47969 = t10523 * t3354;
    let t47973 = F::cast_from(0.8638e0_f64) * t47951 + F::cast_from(0.21595e0_f64) * t47955 - F::cast_from(0.10664197530864197531e0_f64) * t47959 + F::cast_from(0.47988888888888888888e-1_f64) * t40954 + F::cast_from(0.53320987654320987654e-1_f64) * t40956 - F::cast_from(0.19195555555555555555e0_f64) * t40958 + F::cast_from(0.28793333333333333333e0_f64) * t40960 + F::cast_from(0.79012345679012345678e-1_f64) * t24980 + F::cast_from(0.88888888888888888889e-1_f64) * t32405 + F::cast_from(0.79012345679012345679e-2_f64) * t41939 + F::cast_from(0.17777777777777777778e-1_f64) * t41941 + F::cast_from(0.79999999999999999998e-1_f64) * t25 * t1714 * t47969;
    (t47949, t47951, t47953, t47955, t47957, t47959, t47969, t47973)
}
