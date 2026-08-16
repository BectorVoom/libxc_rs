//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 956/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk956<F: Float>(t10777: F, t418: F, t1856: F, t10756: F, t10758: F, t10760: F, t10763: F, t10771: F, t10774: F, t25: F, t4941: F, t5241: F, t5256: F, t5271: F, t7335: F, t7364: F, t7374: F, t7376: F, t7379: F, t7380: F) -> (F, F) {
    let t10778 = t10777 * t418;
    let t10779 = t1856 * t10778;
    let t10782 = F::cast_from(0.44444444444444444445e-2_f64) * t10756 + F::cast_from(0.14814814814814814815e-2_f64) * t10758 - F::cast_from(0.88888888888888888887e-2_f64) * t10760 - F::cast_from(0.66666666666666666667e-2_f64) * t25 * t10763 - F::cast_from(0.15996296296296296296e-1_f64) * t4941 - t5241 + t7335 - t7364 - t5271 - F::cast_from(0.31992592592592592592e-1_f64) * t7374 + F::cast_from(0.47988888888888888888e-1_f64) * t7376 + t7379 - F::cast_from(0.47988888888888888888e-1_f64) * t7380 - F::cast_from(0.74074074074074074073e-2_f64) * t5256 + F::cast_from(0.13333333333333333333e-1_f64) * t25 * t10771 - F::cast_from(0.22222222222222222222e-2_f64) * t25 * t10774 + F::cast_from(0.13333333333333333333e-1_f64) * t25 * t10779;
    (t10778, t10782)
}
