//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 656/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk656<F: Float>(t3793: F, t3808: F, t3812: F, t3814: F, t418: F, t4329: F, t4399: F, t4472: F, t4507: F, t4570: F, t4613: F, t4656: F, t4690: F, t4767: F, t4899: F, t4945: F, t5008: F, t5155: F, t5197: F, t5225: F, t5226: F, t5229: F, t5232: F, t5237: F, t5240: F, t5243: F, t5294: F) -> F {
    let t5299 = t4767 + t4945 + t5155 + t3808 + t4613 + F::new(0.17149607247227894789e-2) * t3812 - F::new(7.0) / F::new(288.0) * t3814 - F::new(0.16006300097412701803e-1) * t3793 - t5229 + t4329 + t4399 + t4690 + t5243 - F::new(0.42874018118069736972e-3) * t5226 + t4656 + t5197 + F::new(0.42874018118069736972e-3) * t5240 + t4899 + t4570 + t4507 + t4472 + t5225 - F::new(0.42874018118069736972e-3) * t418 * t5232 - F::new(0.85748036236139473944e-3) * t418 * t5237 + t5008 + t5294;
    t5299
}
