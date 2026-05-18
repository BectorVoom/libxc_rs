//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1156/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1156<F: Float>(t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F, t34269: F, t34274: F, t2967: F, t3179: F, t4915: F) -> (F, F) {
    let t34276 = F::new(0.51491428373437201895e-6) * t34235 + F::new(0.20010856351627032588e-8) * t34238 + F::new(0.17376185052903442709e-3) * t34241 + F::new(0.24581606547037760418e-8) * t34245 - F::new(0.81938688490125868062e-9) * t34249 - F::new(0.51491428373437201896e-5) * t34252 - F::new(0.16387737698025173612e-8) * t34255 + F::new(0.11049275749843950005e-7) * t34258 + F::new(0.66295654499063700028e-7) * t34264 - F::new(0.54785992259642918774e-7) * t34269 + F::new(0.39291224566445086216e-8) * t34274;
    let t34285 = F::new(24.0) * t4915 * t2967 * t3179;
    (t34276, t34285)
}
