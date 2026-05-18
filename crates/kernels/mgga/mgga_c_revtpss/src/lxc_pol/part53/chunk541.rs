//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 541/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk541<F: Float>(t45: F, t57: F, t1469: F, t2375: F, t4186: F, t606: F, t78: F, t2382: F, t81: F, t162: F, t187: F, t150: F, t190: F, t1532: F, t750: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t4377 = t2375 * t1469;
    let t4383 = piecewise3::<f64>(t151, F::new(0.0), F::new(4.0) / F::new(9.0) * t4377 * t606 + F::new(4.0) / F::new(3.0) * t78 * t4186);
    let t4384 = t2382 * t1469;
    let t4390 = piecewise3::<f64>(t155, F::new(0.0), F::new(4.0) / F::new(9.0) * t4384 * t606 - F::new(4.0) / F::new(3.0) * t81 * t4186);
    let t4391 = t4383 + t4390;
    let t4392 = t4391 * t162;
    let t4394 = F::new(0.19751673498613801407e-1) * t4392 * t187;
    let t4395 = t150 * t4391;
    let t4396 = t4395 * t190;
    let t4397 = t1532 * t750;
    (t4394, t4396, t4397)
}
