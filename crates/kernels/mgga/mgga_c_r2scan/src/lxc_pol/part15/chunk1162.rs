//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1162/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1162<F: Float>(t3602: F, t37755: F, t6064: F, t3606: F, t6087: F, t37983: F, t39900: F, t39903: F, t39906: F, t39908: F, t39912: F, t39914: F, t39916: F, t39920: F, t39924: F) -> F {
    let t39927 = t37755 * t3602 * t6064;
    let t39930 = t37755 * t3606 * t6087;
    let t39932 = -t39900 - F::new(0.13972381860938637373e0) * t39903 + F::new(0.67533178994536747305e0) * t39906 - F::new(0.32927245914677557993e-1) * t39908 - t39912 - F::new(0.5200933044032561138e0) * t39914 - F::new(0.54878743191129263322e-1) * t39916 + F::new(0.19514881078765566037e-1) * t37983 + F::new(0.14282990759302185291e-1) * t39920 + F::new(0.87327386630866483584e-2) * t39924 + F::new(0.13099107994629972538e-1) * t39927 + F::new(0.13099107994629972538e-1) * t39930;
    t39932
}
