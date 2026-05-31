//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 996/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk996<F: Float>(t45: F, t57: F, t10326: F, t10356: F, t10472: F, t2258: F, t633: F, t766: F, t606: F, t83: F, t637: F, t770: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10478 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t633 * t10356 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10472 * t2258 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t10326);
    let t10481 = t83 * t606;
    let t10487 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t637 * t10356 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10481 * t2258 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t10326);
    let t10489 = t10478 / F::cast_from(2.0_f64) + t10487 / F::cast_from(2.0_f64);
    (t10481, t10489)
}
