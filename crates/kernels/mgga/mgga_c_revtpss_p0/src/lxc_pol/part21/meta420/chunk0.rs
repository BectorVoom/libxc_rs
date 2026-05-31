//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1907/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1907<F: Float>(t33: F, t1711: F, t9350: F, t2: F, t3841: F, t1113: F, t580: F, t22: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t13565 = t9350 * t1711;
    let t13568 = t3841 * t2;
    let t13569 = t580 * t1113;
    let t13579 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13565 * t3842 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13568 * t13569 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5557 * t3351 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t516 * t580 + F::cast_from(8.0_f64) * t5560 * t22);
    (t13565, t13568, t13569, t13579)
}
