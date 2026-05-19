//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1174/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1174<F: Float>(t1030: F, t144: F, t33521: F, t34546: F, t4052: F, t34515: F, t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34530: F, t34533: F, t34537: F, t34539: F) -> F {
    let t34547 = t1030 * t4052 * t33521 * t144 * t34546;
    let t34549 = F::cast_from(0.25301920572916666668e-5_f64) * t34515 + F::cast_from(0.12650960286458333334e-5_f64) * t34517 + F::cast_from(0.25301920572916666668e-5_f64) * t34520 + F::cast_from(0.12650960286458333334e-5_f64) * t34522 - F::cast_from(0.25301920572916666668e-5_f64) * t34525 - F::cast_from(0.24458523220486111112e-4_f64) * t34528 + F::cast_from(0.2845640240200497334e-7_f64) * t34530 + F::cast_from(0.34380927311705569432e-8_f64) * t34533 - F::cast_from(0.65555167711046006955e-8_f64) * t34537 + F::cast_from(0.70344136651018351214e-8_f64) * t34539 + F::cast_from(0.28199579487947481489e-8_f64) * t34547;
    t34549
}
