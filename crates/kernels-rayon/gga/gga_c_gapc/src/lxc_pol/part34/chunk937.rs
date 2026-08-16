//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 937/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk937(t3622: f64, t972: f64, t1125: f64, t2470: f64, t9384: f64, t9391: f64, t9393: f64, t9397: f64, t9400: f64, t9403: f64, t9406: f64, t9409: f64, t9412: f64, t9416: f64, t9420: f64, t9423: f64, t9426: f64) -> (f64, f64, f64) {
    let t10791 = t3622 * t972;
    let t10794 = t1125 * t2470;
    let t10813 = -0.28960308421505737848e-5_f64 * t9384 + 0.39476761752968521453e-4_f64 * t9391 - 0.4637672555408563478e-4_f64 * t9393 - 0.29517957899305555558e-5_f64 * t9397 - 0.17989505234049721814e-7_f64 * t9400 + 0.23989005229605304038e-7_f64 * t9403 - 0.17376185052903442709e-3_f64 * t9406 - 0.9275345110817126956e-4_f64 * t9409 - 0.77294542590142724634e-6_f64 * t9412 + 0.33351427252711720978e-8_f64 * t9416 + 0.16413103962948681584e-7_f64 * t9420 + 0.20240885416666666668e-4_f64 * t9423 - 0.10120442708333333334e-4_f64 * t9426;
    (t10791, t10794, t10813)
}
