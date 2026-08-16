//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 916/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk916(t10395: f64, t3231: f64, t2300: f64, t493: f64, t3217: f64, t1061: f64, t6925: f64, t3239: f64, t6927: f64, t10371: f64, t10374: f64, t10376: f64, t10379: f64, t10383: f64, t10386: f64, t10390: f64, t10393: f64) -> (f64, f64, f64, f64) {
    let t10396 = t10395 * t3231;
    let t10398 = t493 * t2300;
    let t10399 = t3217 * t10398;
    let t10401 = t1061 * t6925;
    let t10402 = t3239 * t6927;
    let t10403 = t10401 * t10402;
    let t10405 = -0.3556532540941297432e-4_f64 * t10371 + 0.41036913933938047292e-5_f64 * t10374 + 0.93943849568165663176e-3_f64 * t10376 + 0.43840463131810642816e-4_f64 * t10379 - 0.56366309740899397906e-3_f64 * t10383 + 0.93943849568165663176e-3_f64 * t10386 - 0.13298999885998441252e-6_f64 * t10390 - 0.41036913933938047292e-5_f64 * t10393 - 0.82073827867876094584e-5_f64 * t10396 - 0.41036913933938047292e-5_f64 * t10399 - 0.11742981196020707897e-4_f64 * t10403;
    (t10396, t10399, t10403, t10405)
}
