//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 801/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk801<F: Float>(t1218: F, t1232: F, t2134: F, t2136: F, t24704: F, t24706: F, t24712: F, t24716: F, t24723: F, t24729: F, t24733: F, t24736: F, t24741: F, t24747: F, t24749: F, t24752: F, t24754: F, t3496: F, t3511: F, t3518: F, t3527: F, t3531: F, t3580: F, t7339: F, t7345: F) -> F {
    let t24756 = -t24704 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t24706 - t7345 * t3527 / F::cast_from(2304.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t24712 - t7345 * t3531 / F::cast_from(1152.0_f64) + t24716 * t1218 / F::cast_from(768.0_f64) + F::cast_from(0.20186378047070195428e-3_f64) * t24723 + t7339 * t3496 / F::cast_from(1536.0_f64) + t24729 * t3511 / F::cast_from(768.0_f64) - t24733 * t3518 / F::cast_from(1536.0_f64) - t24736 * t1232 / F::cast_from(1152.0_f64) - t24741 * t3580 / F::cast_from(1152.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t24747 - F::cast_from(0.10093189023535097714e-3_f64) * t24749 * t2136 - t24752 / F::cast_from(1728.0_f64) + t24754 / F::cast_from(1152.0_f64);
    t24756
}
