//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 968/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk968<F: Float>(t401: F, t5049: F, t1251: F, t1718: F, t5034: F, t16964: F, t16966: F, t16968: F, t16976: F, t16981: F, t16987: F, t16989: F, t16997: F, t17018: F, t25: F, t657: F) -> F {
    let t17919 = t401 * t5049;
    let t17927 = t1251 * t1718;
    let t17929 = t401 * t5034;
    let t17931 = -F::new(0.63985185185185185184e-1) * t16964 + F::new(0.47988888888888888888e-1) * t16966 + F::new(0.53320987654320987654e-1) * t16968 - F::new(0.10664197530864197531e0) * t16976 - F::new(0.35991666666666666667e-1) * t16981 + F::new(0.21595e0) * t16989 - F::new(0.86380000000000000002e0) * t16997 + F::new(0.17777777777777777778e-1) * t17919 + F::new(0.16e0) * t25 * t657 * t17018 + F::new(0.39999999999999999999e-1) * t25 * t657 * t16987 + F::new(0.88888888888888888889e-1) * t17927 + F::new(0.10666666666666666667e0) * t17929;
    t17931
}
