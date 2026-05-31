//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1142/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1142<F: Float>(t10471: F, t8392: F, t10510: F, t824: F, t9571: F, t10448: F, t10453: F, t10495: F, t2349: F, t2801: F, t10447: F, t10452: F, t10485: F, t10492: F, t15290: F, t1901: F, t2682: F, t2874: F, t2881: F, t4139: F, t4140: F, t42404: F, t4265: F, t43938: F) -> (F, F, F) {
    let t43986 = t8392 * t10471;
    let t43992 = t8392 * t10510;
    let t43999 = t9571 * t824;
    let t44007 = t8392 * t10448;
    let t44009 = t8392 * t10453;
    let t44011 = t8392 * t10495;
    let t44013 = t2349 * t2801;
    let t44021 = -F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43986 - F::cast_from(4.0_f64) * t1901 * t2881 * t4140 * t42404 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43992 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10492 * t4265 * t2349 * t2682 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2874 * t4140 * t43999 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10447 * t10452 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t44007 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t44009 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t44011 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t4139 * t10485 * t44013 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t15290 * t43938;
    (t43999, t44013, t44021)
}
