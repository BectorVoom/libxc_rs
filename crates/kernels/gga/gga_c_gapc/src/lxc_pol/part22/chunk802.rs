//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 802/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk802<F: Float>(t1022: F, t9209: F, t9174: F, t9177: F, t9182: F, t9186: F, t9188: F, t9190: F, t9192: F, t9195: F, t9198: F, t9201: F, t9205: F, t9207: F) -> (F, F) {
    let t9210 = t1022 * t9209;
    let t9212 = -F::new(0.69504740211613770836e-4) * t9174 - F::new(0.69504740211613770836e-4) * t9177 + F::new(0.10005749997240850276e-8) * t9182 + F::new(0.2085142206348413125e-3) * t9186 - F::new(0.2318836277704281739e-4) * t9188 + F::new(0.4637672555408563478e-4) * t9190 + F::new(0.2318836277704281739e-4) * t9192 + F::new(0.38647271295071362318e-6) * t9195 - F::new(0.687148483626368822e-6) * t9198 + F::new(0.86880925264517213544e-4) * t9201 - F::new(0.14480154210752868924e-5) * t9205 + F::new(0.17376185052903442709e-3) * t9207 + F::new(0.17376185052903442709e-3) * t9210;
    (t9210, t9212)
}
