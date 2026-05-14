//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1005/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1005<F: Float>(t824: F, t9571: F, t10448: F, t8392: F, t10453: F, t10495: F, t2349: F, t2801: F, t10447: F, t10452: F, t10485: F, t10492: F, t15290: F, t1901: F, t2682: F, t2874: F, t2881: F, t4139: F, t4140: F, t42404: F, t4265: F, t43938: F, t43986: F, t43992: F) -> (F, F, F) {
    let t43999 = t9571 * t824;
    let t44007 = t8392 * t10448;
    let t44009 = t8392 * t10453;
    let t44011 = t8392 * t10495;
    let t44013 = t2349 * t2801;
    let t44021 = -8.0 / 9.0 * t43986 - 4.0 * t1901 * t2881 * t4140 * t42404 + 8.0 / 9.0 * t43992 + 8.0 / 3.0 * t1901 * t10492 * t4265 * t2349 * t2682 + 8.0 / 3.0 * t1901 * t2874 * t4140 * t43999 - 8.0 / 3.0 * t1901 * t10447 * t10452 - 8.0 / 9.0 * t44007 + 8.0 / 9.0 * t44009 + 8.0 / 9.0 * t44011 + 4.0 / 9.0 * t1901 * t4139 * t10485 * t44013 + 8.0 / 9.0 * t1901 * t15290 * t43938;
    (t43999, t44013, t44021)
}
