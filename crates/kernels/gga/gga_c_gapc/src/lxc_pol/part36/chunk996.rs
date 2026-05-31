//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 996/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk996<F: Float>(t12335: F, t3859: F, t4908: F, t687: F, t4915: F, t1112: F, t3537: F, t1616: F, t1611: F, t3873: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12336 = F::cast_from(2.0_f64) * t12335;
    let t12337 = t4908 * t3859;
    let t12338 = F::cast_from(2.0_f64) * t12337;
    let t12339 = t3859 * t687;
    let t12340 = t4915 * t12339;
    let t12341 = F::cast_from(6.0_f64) * t12340;
    let t12342 = t1112 * t3537;
    let t12343 = t1616 * t12342;
    let t12344 = F::cast_from(4.0_f64) * t12343;
    let t12345 = t1611 * t3873;
    let t12346 = t3873 * t687;
    let t12347 = t1616 * t12346;
    (t12336, t12337, t12338, t12339, t12340, t12341, t12342, t12343, t12344, t12345, t12346, t12347)
}
