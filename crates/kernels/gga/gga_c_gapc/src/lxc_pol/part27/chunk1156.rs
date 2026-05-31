//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1156/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1156<F: Float>(t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F, t34269: F, t34274: F, t2967: F, t3179: F, t4915: F) -> (F, F) {
    let t34276 = F::cast_from(0.51491428373437201895e-6_f64) * t34235 + F::cast_from(0.20010856351627032588e-8_f64) * t34238 + F::cast_from(0.17376185052903442709e-3_f64) * t34241 + F::cast_from(0.24581606547037760418e-8_f64) * t34245 - F::cast_from(0.81938688490125868062e-9_f64) * t34249 - F::cast_from(0.51491428373437201896e-5_f64) * t34252 - F::cast_from(0.16387737698025173612e-8_f64) * t34255 + F::cast_from(0.11049275749843950005e-7_f64) * t34258 + F::cast_from(0.66295654499063700028e-7_f64) * t34264 - F::cast_from(0.54785992259642918774e-7_f64) * t34269 + F::cast_from(0.39291224566445086216e-8_f64) * t34274;
    let t34285 = F::cast_from(24.0_f64) * t4915 * t2967 * t3179;
    (t34276, t34285)
}
