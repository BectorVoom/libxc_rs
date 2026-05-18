//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1292/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1292<F: Float>(t8219: F, t9847: F, t8009: F, t9850: F, t11234: F, t6142: F, t851: F, t2240: F, t3069: F, t3740: F, t18427: F, t18596: F, t22230: F, t22693: F, t27262: F, t27295: F, t31067: F, t31088: F, t378: F) -> (F, F, F, F, F) {
    let t31456 = F::new(18.0) * t8219 * t9847;
    let t31458 = F::new(12.0) * t8009 * t9850;
    let t31461 = F::new(24.0) * t6142 * t11234 * t851;
    let t31464 = F::new(18.0) * t2240 * t3740 * t3069;
    let t31472 = (t18596 - F::new(0.28842592592592592592e-1) * t18427 - F::new(0.86527777777777777779e-1) * t22230 + t22693 + F::new(0.37083333333333333333e-1) * t27295 - F::new(0.278125e-1) * t27262 - F::new(0.92708333333333333333e-2) * t31067 + F::new(0.278125e-1) * t31088) * t378;
    (t31456, t31458, t31461, t31464, t31472)
}
