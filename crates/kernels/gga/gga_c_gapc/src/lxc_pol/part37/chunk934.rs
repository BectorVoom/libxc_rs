//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 934/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk934<F: Float>(t9174: F, t9177: F, t9182: F, t9186: F, t9188: F, t9190: F, t9192: F, t9195: F, t9198: F, t9201: F, t9205: F, t9207: F, t9210: F) -> F {
    let t10738 = -F::new(0.13900948042322754167e-3) * t9174 - F::new(0.13900948042322754167e-3) * t9177 + F::new(0.20011499994481700553e-8) * t9182 + F::new(0.41702844126968262501e-3) * t9186 - F::new(0.4637672555408563478e-4) * t9188 + F::new(0.9275345110817126956e-4) * t9190 + F::new(0.4637672555408563478e-4) * t9192 + F::new(0.77294542590142724634e-6) * t9195 - F::new(0.1374296967252737644e-5) * t9198 + F::new(0.17376185052903442709e-3) * t9201 - F::new(0.28960308421505737848e-5) * t9205 + F::new(0.34752370105806885418e-3) * t9207 + F::new(0.34752370105806885418e-3) * t9210;
    t10738
}
