//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1031/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1031<F: Float>(t115292: F, t115294: F, t115308: F, t115318: F, t117128: F, t122102: F, t122107: F, t122110: F, t122112: F, t122117: F, t122121: F, t122127: F, t122131: F, t122133: F, t33844: F, t3882: F) -> F {
    let t124018 = -F::cast_from(0.15352717957250113407e0_f64) * t122102 - t3882 * t33844 + F::cast_from(0.76763589786250567037e-1_f64) * t115292 + F::cast_from(0.6579736267392905746e-1_f64) * t122107 + F::cast_from(0.6579736267392905746e-1_f64) * t122110 - F::cast_from(0.15352717957250113407e0_f64) * t122112 + F::cast_from(0.76763589786250567037e-1_f64) * t115294 + F::cast_from(0.6579736267392905746e-1_f64) * t122117 - t117128 + F::cast_from(0.16449340668482264365e-1_f64) * t122121 + F::cast_from(0.16449340668482264365e-1_f64) * t115308 + F::cast_from(0.6579736267392905746e-1_f64) * t122127 + F::cast_from(0.6579736267392905746e-1_f64) * t122131 + F::cast_from(0.76763589786250567037e-1_f64) * t122133 - F::cast_from(0.3289868133696452873e-1_f64) * t115318;
    t124018
}
