//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 803/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk803(t1022: f64, t9209: f64, t9174: f64, t9177: f64, t9182: f64, t9186: f64, t9188: f64, t9190: f64, t9192: f64, t9195: f64, t9198: f64, t9201: f64, t9205: f64, t9207: f64) -> (f64, f64) {
    let t9210 = t1022 * t9209;
    let t9212 = -0.69504740211613770836e-4_f64 * t9174 - 0.69504740211613770836e-4_f64 * t9177 + 0.10005749997240850276e-8_f64 * t9182 + 0.2085142206348413125e-3_f64 * t9186 - 0.2318836277704281739e-4_f64 * t9188 + 0.4637672555408563478e-4_f64 * t9190 + 0.2318836277704281739e-4_f64 * t9192 + 0.38647271295071362318e-6_f64 * t9195 - 0.687148483626368822e-6_f64 * t9198 + 0.86880925264517213544e-4_f64 * t9201 - 0.14480154210752868924e-5_f64 * t9205 + 0.17376185052903442709e-3_f64 * t9207 + 0.17376185052903442709e-3_f64 * t9210;
    (t9210, t9212)
}
