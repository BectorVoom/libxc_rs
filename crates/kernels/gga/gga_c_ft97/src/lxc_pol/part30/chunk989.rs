//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 989/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk989<F: Float>(t1173: F, t7440: F, t24412: F, t27924: F, t10052: F, t1131: F, t1403: F, t140605: F, t193: F, t24191: F, t27953: F, t27965: F, t27974: F, t27991: F, t33243: F, t33253: F, t33568: F, t35737: F, t6002: F, t6008: F, t6192: F, t6754: F, t684: F, t6945: F, t713: F, t7437: F, t766: F, t9770: F) -> (F, F) {
    let t149832 = t7440 * t1173;
    let t149837 = t24412 * t27924;
    let t149843 = -t140605 / F::new(3.0) - t7437 * t27965 / F::new(3.0) + t1403 * t193 * t33243 * t27974 - t33568 * t6754 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1403 * t193 * t6008 * t6192 * t1131 - t1403 * t193 * t33253 * t27974 / F::new(3.0) - F::new(24.0) * t10052 * t35737 * t766 - F::new(2.0) / F::new(3.0) * t1403 * t193 * t6008 * t6945 * t713 - t7437 * t27953 / F::new(3.0) + t6002 * t9770 * t149832 * t684 / F::new(9.0) + F::new(8.0) * t149837 - F::new(2.0) / F::new(3.0) * t1403 * t193 * t24191 * t27991;
    (t149837, t149843)
}
