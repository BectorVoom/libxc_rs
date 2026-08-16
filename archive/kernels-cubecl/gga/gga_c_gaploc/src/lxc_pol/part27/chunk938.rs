//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 938/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk938<F: Float>(t10293: F, t3366: F, t605: F, t4349: F, t2902: F, t921: F, t1382: F, t1016: F, t2497: F, t3381: F, t4379: F, t2366: F, t2754: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10294 = F::cast_from(2.0_f64) * t10293;
    let t10295 = t3366 * t605;
    let t10296 = t4349 * t10295;
    let t10297 = F::cast_from(6.0_f64) * t10296;
    let t10298 = t2902 * t921;
    let t10299 = t1382 * t10298;
    let t10300 = F::cast_from(2.0_f64) * t10299;
    let t10301 = t1016 * t2497;
    let t10302 = t1382 * t10301;
    let t10303 = F::cast_from(2.0_f64) * t10302;
    let t10308 = t4379 * t3381;
    let t10309 = F::cast_from(0.14896037479937677779e-1_f64) * t10308;
    let t10310 = t2366 * t2754;
    (t10294, t10295, t10296, t10297, t10298, t10299, t10300, t10301, t10302, t10303, t10309, t10310)
}
