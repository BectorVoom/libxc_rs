//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1279/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1279<F: Float>(t3886: F, t1109: F, t1115: F, t12130: F, t12232: F, t13112: F, t13187: F, t2409: F, t2416: F, t2501: F, t3055: F, t3207: F, t335: F, t338: F, t35000: F, t353: F, t35941: F, t3780: F, t3921: F, t43290: F, t44091: F, t44093: F, t44118: F, t44131: F, t44138: F, t829: F, t830: F, t9885: F, t9899: F) -> F {
    let t50499 = t3886 * t3886;
    let t50514 = F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t35941 + t12130 * t829 * t830 * t2501 * t3780 / F::cast_from(8.0_f64) - t3921 * t9885 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t44091 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t44093 - t3055 * t829 * t830 * t12232 * t1109 / F::cast_from(16.0_f64) - t3921 * t9899 / F::cast_from(16.0_f64) + t35000 * t13112 / F::cast_from(4.0_f64) + t335 * t338 * t353 * t2416 * t50499 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3207 * t2409 * t2501 * t13187 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t44118 - t1115 * t43290 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t44131 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t44138;
    t50514
}
