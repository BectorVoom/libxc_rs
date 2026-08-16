//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3267/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3267<F: Float>(t33: F, t1113: F, t13565: F, t13568: F, t20256: F, t21918: F, t2255: F, t22778: F, t22783: F, t3841: F, t47040: F, t516: F, t5557: F, t81123: F, t85426: F, t85429: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t85968 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t47040 * t22778 * t1113 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21918 * t2255 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13565 * t85426 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t13568 * t85429 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5557 * t20256 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3841 * t22783 * t1113 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t516 * t81123);
    t85968
}
