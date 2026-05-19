//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1412/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1412<F: Float>(t12042: F, t12591: F, t12586: F, t12655: F, t12622: F, t1616: F, t687: F, t35397: F, t36332: F, t36333: F, t36334: F, t36335: F, t36336: F, t36337: F, t36338: F, t36340: F, t36341: F, t36342: F, t36343: F, t36344: F) -> (F, F, F, F, F, F) {
    let t37330 = F::new(2.0) * t12042;
    let t38531 = F::new(4.0) * t12591;
    let t38532 = F::new(2.0) * t12586;
    let t38534 = F::new(2.0) * t12655;
    let t38537 = F::new(4.0) * t1616 * t12622 * t687;
    let t38539 = t36332 - t36333 - t36334 - t36335 - t36336 - t36337 - t36338 + F::cast_from(0.53949325746737929041e-3_f64) * t35397 - t36340 - t36341 + t36342 - t36343 - t36344;
    (t37330, t38531, t38532, t38534, t38537, t38539)
}
