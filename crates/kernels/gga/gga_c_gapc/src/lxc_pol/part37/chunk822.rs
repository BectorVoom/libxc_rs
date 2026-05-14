//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 822/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk822<F: Float>(t10388: F, t2536: F, t10343: F, t2405: F, t493: F, t3230: F, t6808: F, t996: F, t3231: F, t2300: F, t3217: F, t1061: F, t6925: F, t3239: F, t6927: F, t10371: F, t10374: F, t10376: F, t10379: F, t10383: F, t10386: F) -> (F, F, F, F, F, F) {
    let t10389 = t10388 * t2536;
    let t10390 = t10343 * t10389;
    let t10392 = t493 * t2405;
    let t10393 = t3230 * t10392;
    let t10395 = t996 * t6808;
    let t10396 = t10395 * t3231;
    let t10398 = t493 * t2300;
    let t10399 = t3217 * t10398;
    let t10401 = t1061 * t6925;
    let t10402 = t3239 * t6927;
    let t10403 = t10401 * t10402;
    let t10405 = -0.3556532540941297432e-4 * t10371 + 0.41036913933938047292e-5 * t10374 + 0.93943849568165663176e-3 * t10376 + 0.43840463131810642816e-4 * t10379 - 0.56366309740899397906e-3 * t10383 + 0.93943849568165663176e-3 * t10386 - 0.13298999885998441252e-6 * t10390 - 0.41036913933938047292e-5 * t10393 - 0.82073827867876094584e-5 * t10396 - 0.41036913933938047292e-5 * t10399 - 0.11742981196020707897e-4 * t10403;
    (t10390, t10393, t10396, t10399, t10403, t10405)
}
