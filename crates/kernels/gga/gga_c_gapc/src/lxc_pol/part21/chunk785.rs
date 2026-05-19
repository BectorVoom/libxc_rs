//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 785/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk785<F: Float>(t1022: F, t9209: F, t9174: F, t9177: F, t9182: F, t9186: F, t9188: F, t9190: F, t9192: F, t9195: F, t9198: F, t9201: F, t9205: F, t9207: F) -> F {
    let t9210 = t1022 * t9209;
    let t9212 = -F::cast_from(0.69504740211613770836e-4_f64) * t9174 - F::cast_from(0.69504740211613770836e-4_f64) * t9177 + F::cast_from(0.10005749997240850276e-8_f64) * t9182 + F::cast_from(0.2085142206348413125e-3_f64) * t9186 - F::cast_from(0.2318836277704281739e-4_f64) * t9188 + F::cast_from(0.4637672555408563478e-4_f64) * t9190 + F::cast_from(0.2318836277704281739e-4_f64) * t9192 + F::cast_from(0.38647271295071362318e-6_f64) * t9195 - F::cast_from(0.687148483626368822e-6_f64) * t9198 + F::cast_from(0.86880925264517213544e-4_f64) * t9201 - F::cast_from(0.14480154210752868924e-5_f64) * t9205 + F::cast_from(0.17376185052903442709e-3_f64) * t9207 + F::cast_from(0.17376185052903442709e-3_f64) * t9210;
    t9212
}
