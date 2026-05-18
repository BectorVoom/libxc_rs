//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 602/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk602<F: Float>(t3129: F, t3134: F, t3145: F, t3147: F, t3150: F, t3158: F, t3161: F, t3164: F, t3166: F, t3168: F, t3173: F, t3175: F) -> F {
    let t3535 = -F::new(0.75883739738679928909e-7) * t3129 + F::new(0.1349212892553729136e-6) * t3134 - F::new(0.49240895655712845849e-7) * t3145 + F::new(0.27801896084645508334e-2) * t3147 + F::new(0.20241536458333333335e-4) * t3150 + F::new(0.29518907335069444447e-5) * t3158 + F::new(0.27801896084645508334e-2) * t3161 - F::new(0.28985453471303521736e-5) * t3164 - F::new(0.10120768229166666668e-3) * t3166 + F::new(0.12380568050579229813e-5) * t3168 - F::new(0.69504740211613770835e-4) * t3173 - F::new(0.64871090864172852779e-2) * t3175;
    t3535
}
