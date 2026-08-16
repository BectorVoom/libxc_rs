//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 876/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk876<F: Float>(t1162: F, t338: F, t3907: F, t1115: F, t12111: F, t12195: F, t12199: F, t12223: F, t12246: F, t12253: F, t13641: F, t13645: F, t13650: F, t13656: F, t13662: F, t13680: F, t13684: F, t13688: F, t2401: F, t2408: F, t2503: F, t335: F, t3921: F, t833: F, t844: F, t8659: F, t9820: F, t9899: F) -> (F, F) {
    let t13695 = t338 * t3907 * t1162;
    let t13698 = -t844 * t13641 / F::cast_from(48.0_f64) - t844 * t13645 / F::cast_from(16.0_f64) - t2408 * t13650 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t12195 - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t12199 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2401 * t13656 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t12223 + t8659 * t13662 / F::cast_from(48.0_f64) + t1115 * t12111 / F::cast_from(16.0_f64) - t1115 * t9899 / F::cast_from(32.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t1115 * t9820 + t335 * t13680 / F::cast_from(96.0_f64) + t2408 * t13684 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t12246 + t13688 * t833 / F::cast_from(48.0_f64) + t3921 * t2503 / F::cast_from(32.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t12253 - t335 * t13695 / F::cast_from(32.0_f64);
    (t13695, t13698)
}
