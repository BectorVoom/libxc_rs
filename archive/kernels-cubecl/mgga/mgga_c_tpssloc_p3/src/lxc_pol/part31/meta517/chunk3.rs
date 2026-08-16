//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1718/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1718<F: Float>(t24230: F, t24231: F, t25109: F, t25126: F, t25133: F, t25140: F, t25144: F, t28380: F, t28384: F, t28386: F, t28390: F, t28397: F, t28399: F, t28401: F, t28403: F) -> F {
    let t29039 = F::cast_from(0.33913115119077928316e-1_f64) * t25109 + t28380 / F::cast_from(96.0_f64) - F::cast_from(0.24223653656484234512e-2_f64) * t28384 + t28386 / F::cast_from(8.0_f64) + F::cast_from(0.16956557559538964158e-1_f64) * t28390 + F::cast_from(0.56521858531796547194e-2_f64) * t25126 + F::cast_from(0.13457585364713463618e-3_f64) * t25133 + F::cast_from(0.48447307312968469024e-2_f64) * t28397 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t25140 - t28399 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t28401 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t25144 - t28403 / F::cast_from(24.0_f64) + t24230 + t24231;
    t29039
}
