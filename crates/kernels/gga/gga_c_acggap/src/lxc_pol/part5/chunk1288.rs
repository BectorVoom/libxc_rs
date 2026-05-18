//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1288/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1288<F: Float>(t6121: F, t997: F, t1886: F, t3228: F, t1891: F, t1008: F, t5690: F, t1165: F, t1180: F, t1181: F, t18475: F, t18480: F, t18482: F, t18485: F, t18487: F, t18489: F, t22710: F, t3176: F, t3462: F, t530: F, t5922: F) -> F {
    let t23891 = t997 * t6121;
    let t23893 = t3228 * t1886;
    let t23895 = t3228 * t1891;
    let t23897 = t1008 * t5690;
    let t23909 = F::new(0.85748036236139473944e-3) * t1180 * t1165 * t5922 * t3176 - F::new(0.40015750243531754508e-2) * t23891 + F::new(0.42874018118069736972e-2) * t23893 - F::new(0.85748036236139473944e-3) * t23895 - F::new(0.17149607247227894789e-2) * t23897 - F::new(0.68598428988911579156e-2) * t18475 - F::new(0.34299214494455789578e-2) * t18480 + F::new(0.13719685797782315831e-1) * t18482 + F::new(0.68598428988911579156e-2) * t18485 + F::new(7.0) / F::new(72.0) * t18487 + F::new(7.0) / F::new(144.0) * t18489 - F::new(0.68598428988911579156e-2) * t3462 * t1181 * t530 * t22710;
    t23909
}
