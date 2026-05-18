//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1276/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1276<F: Float>(t12743: F, t1750: F, t14220: F, t6144: F, t1165: F, t1180: F, t14044: F, t14048: F, t14054: F, t14059: F, t18159: F, t18164: F, t18166: F, t18176: F, t18189: F, t1884: F, t3403: F, t4437: F, t5922: F, t955: F) -> F {
    let t23593 = t12743 * t1750;
    let t23606 = t14220 * t6144;
    let t23614 = F::new(0.45351183609335988442e-1) * t23593 + F::new(0.40015750243531754508e-2) * t18159 + F::new(0.21437009059034868486e-3) * t14044 - F::new(0.42874018118069736972e-2) * t3403 * t1165 * t1884 * t955 - F::new(0.13719685797782315831e-1) * t18164 - F::new(0.64025200389650807212e-1) * t18166 - F::new(0.68598428988911579156e-2) * t14048 - F::new(0.12004725073059526352e-1) * t14054 + F::new(0.68598428988911579156e-2) * t14059 - F::new(0.16006300097412701803e-1) * t23606 - F::new(0.85748036236139473944e-3) * t18176 - F::new(0.17149607247227894789e-1) * t18189 + F::new(0.42874018118069736972e-3) * t1180 * t1165 * t5922 * t4437;
    t23614
}
