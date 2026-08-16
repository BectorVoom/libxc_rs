//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 950/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk950<F: Float>(t1403: F, t2399: F, t7486: F, t7442: F, t33568: F, t5999: F, t140768: F, t141200: F, t141203: F, t141363: F, t141367: F, t24211: F, t7437: F) -> (F, F, F, F, F, F, F, F, F) {
    let t141543 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1403 * t2399 * t7486;
    let t141552 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1403 * t2399 * t7442;
    let t141560 = t33568 * t5999;
    let t141577 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t140768;
    let t141606 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t141200;
    let t141607 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t141203;
    let t141651 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t141363;
    let t141652 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t141367;
    let t141671 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7437 * t24211;
    (t141543, t141552, t141560, t141577, t141606, t141607, t141651, t141652, t141671)
}
