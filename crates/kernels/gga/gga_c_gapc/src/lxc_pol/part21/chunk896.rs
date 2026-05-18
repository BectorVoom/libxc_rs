//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 896/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk896<F: Float>(t10395: F, t3231: F, t2300: F, t493: F, t3217: F, t1061: F, t6925: F, t3239: F, t6927: F, t10371: F, t10374: F, t10376: F, t10379: F, t10383: F, t10386: F, t10390: F, t10393: F) -> F {
    let t10396 = t10395 * t3231;
    let t10398 = t493 * t2300;
    let t10399 = t3217 * t10398;
    let t10401 = t1061 * t6925;
    let t10402 = t3239 * t6927;
    let t10403 = t10401 * t10402;
    let t10405 = -F::new(0.3556532540941297432e-4) * t10371 + F::new(0.41036913933938047292e-5) * t10374 + F::new(0.93943849568165663176e-3) * t10376 + F::new(0.43840463131810642816e-4) * t10379 - F::new(0.56366309740899397906e-3) * t10383 + F::new(0.93943849568165663176e-3) * t10386 - F::new(0.13298999885998441252e-6) * t10390 - F::new(0.41036913933938047292e-5) * t10393 - F::new(0.82073827867876094584e-5) * t10396 - F::new(0.41036913933938047292e-5) * t10399 - F::new(0.11742981196020707897e-4) * t10403;
    t10405
}
