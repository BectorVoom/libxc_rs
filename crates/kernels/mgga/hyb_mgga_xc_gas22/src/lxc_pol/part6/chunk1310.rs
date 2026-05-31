//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1310/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1310<F: Float>(t143: F, t10350: F, t10427: F, t1270: F, t1285: F, t172: F, t187: F, t2104: F, t2147: F, t28335: F, t28376: F, t28414: F, t28456: F, t28459: F, t28585: F, t28623: F, t3227: F, t3267: F, t4046: F, t4082: F, t740: F, t759: F, t8354: F, t8434: F) -> F {
    let t144 = F::cast_from(0.135e1_f64) <= t143;
    let t28628 = piecewise3::<F>(t144, t28335 + t28376 + t28414 + t28456, -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t28459 * t187 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t10350 * t759 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4046 * t2147 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t8354 * t1285 - F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t3227 * t3267 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t1270 * t8434 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2104 * t4082 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t740 * t10427 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t172 * (t28585 + t28623));
    t28628
}
