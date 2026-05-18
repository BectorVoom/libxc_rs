//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 929/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk929<F: Float>(t9097: F, t9100: F, t9104: F, t9106: F, t9108: F, t9111: F, t9115: F, t9118: F, t9121: F, t9124: F, t9126: F, t9130: F, t9132: F) -> F {
    let t10708 = -F::new(0.33816362383187442026e-4) * t9097 + F::new(0.28985453471303521736e-5) * t9100 - F::new(0.91551759647971344971e-6) * t9104 + F::new(0.33816362383187442026e-4) * t9106 - F::new(0.10136107947527008247e-3) * t9108 - F::new(0.10136107947527008247e-3) * t9111 - F::new(0.37516872880543120646e-8) * t9115 + F::new(0.25294579912893309636e-8) * t9118 + F::new(0.12974218172834570556e-1) * t9121 - F::new(0.27801896084645508334e-2) * t9124 + F::new(0.132681342766433194e-5) * t9126 + F::new(0.20241536458333333336e-3) * t9130 + F::new(0.55603792169291016668e-2) * t9132;
    t10708
}
