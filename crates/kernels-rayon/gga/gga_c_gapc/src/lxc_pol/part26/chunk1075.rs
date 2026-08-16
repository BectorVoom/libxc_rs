//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1075/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1075(t33320: f64, t33324: f64, t33326: f64, t33330: f64, t33333: f64, t33336: f64, t33339: f64, t33341: f64, t33343: f64, t33346: f64, t33349: f64, t11902: f64, t19161: f64) -> (f64, f64) {
    let t33351 = 0.21720231316129303386e-4_f64 * t33320 - 0.34752370105806885418e-3_f64 * t33324 - 0.16217772716043213195e-2_f64 * t33326 + 0.71696352428860134554e-9_f64 * t33330 - 0.11594181388521408695e-4_f64 * t33333 - 0.61454016367594401047e-9_f64 * t33336 + 0.81938688490125868062e-9_f64 * t33339 + 0.16217772716043213195e-2_f64 * t33341 - 0.30660168560756614104e-3_f64 * t33343 + 0.11233430345674682505e-6_f64 * t33346 + 0.57970906942607043474e-5_f64 * t33349;
    let t33353 = t11902 * t19161;
    (t33351, t33353)
}
