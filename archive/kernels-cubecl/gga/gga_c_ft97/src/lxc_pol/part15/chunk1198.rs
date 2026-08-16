//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1198/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1198<F: Float>(t22249: F, t55768: F, t22346: F, t4246: F, t1091: F, t1255: F, t21355: F, t22471: F, t2857: F, t2862: F, t446: F, t4969: F, t5225: F, t5393: F, t5424: F, t56665: F, t56689: F, t72167: F, t835: F, t84486: F, t84500: F, t84504: F, t84547: F, t871: F) -> (F, F, F) {
    let t90803 = t55768 * t22249;
    let t90817 = t4246 * t22346;
    let t90863 = -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t84486 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t84500 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t835 * t5424 * t4969 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t446 * t2857 * t1255 * t21355 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t835 * t22471 * t1091 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t84504 - F::cast_from(4.0_f64) * t446 * t2862 * t871 * t5225 * t5393 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t72167 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t56665 - F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t56689 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t84547;
    (t90803, t90817, t90863)
}
