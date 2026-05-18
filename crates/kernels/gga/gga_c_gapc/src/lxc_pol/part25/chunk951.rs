//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 951/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk951<F: Float>(t9944: F, t9946: F, t9948: F, t9953: F, t9955: F, t9960: F, t9962: F, t9964: F, t9967: F, t9970: F, t9973: F, t9978: F, t9981: F) -> F {
    let t11005 = F::new(0.44315380699961440276e-6) * t9944 + F::new(0.30353495895471971564e-6) * t9946 - F::new(0.53968515702149165441e-6) * t9948 - F::new(0.23590742743871821894e-5) * t9953 + F::new(0.12380169846338434109e-5) * t9955 - F::new(0.1778965129659643197e-8) * t9960 + F::new(0.37108289930555555558e-4) * t9962 + F::new(0.16682738775705804733e-3) * t9964 - F::new(0.74147656857749570729e-3) * t9967 + F::new(0.13900948042322754167e-3) * t9970 + F::new(0.13900948042322754167e-3) * t9973 + F::new(0.51491428373437201895e-6) * t9978 - F::new(0.28985453471303521736e-5) * t9981;
    t11005
}
