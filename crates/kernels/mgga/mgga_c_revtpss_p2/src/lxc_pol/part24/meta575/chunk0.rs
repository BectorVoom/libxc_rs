//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1759/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759<F: Float>(t1723: F, t81513: F, t20356: F, t6449: F, t20365: F, t24312: F, t5087: F, t56236: F, t58153: F, t68399: F, t68583: F, t68585: F, t68590: F, t81236: F, t81491: F, t81496: F, t81539: F) -> (F, F, F, F, F) {
    let t90486 = t81513 * t1723;
    let t90488 = t20356 * t6449;
    let t90490 = t20365 * t6449;
    let t90492 = t5087 * t24312;
    let t90497 = -F::cast_from(0.40256666666666666668e0_f64) * t81236 - F::cast_from(0.12524296296296296297e1_f64) * t56236 + F::cast_from(0.16102666666666666667e1_f64) * t68399 - F::cast_from(0.132456e1_f64) * t81491 - F::cast_from(0.98115555555555555555e-1_f64) * t81496 - F::cast_from(0.98115555555555555556e0_f64) * t58153 + F::cast_from(0.22076e0_f64) * t81539 - F::cast_from(0.51785e1_f64) * t90486 + F::cast_from(0.11651625e2_f64) * t90488 - F::cast_from(0.247573125e0_f64) * t90490 + F::cast_from(0.3300975e0_f64) * t90492 + F::cast_from(0.5519e0_f64) * t68583 + F::cast_from(0.11038e1_f64) * t68585 - F::cast_from(0.18396666666666666667e0_f64) * t68590;
    (t90486, t90488, t90490, t90492, t90497)
}
