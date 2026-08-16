//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1150/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1150<F: Float>(t1114: F, t15846: F, t3931: F, t14911: F, t4278: F, t12490: F, t14906: F, t1125: F, t12431: F, t12465: F, t12472: F, t12477: F, t12480: F, t12530: F, t12537: F, t4234: F, t4242: F, t4265: F, t4285: F, t9607: F) -> F {
    let t15868 = t15846 * t1114;
    let t15869 = t3931 * t15868;
    let t15872 = t4278 * t14911;
    let t15873 = t3931 * t15872;
    let t15876 = t12490 * t14906;
    let t15877 = t3931 * t15876;
    let t15880 = -t12465 + t12472 * t4242 / F::cast_from(432.0_f64) - t12477 - t12431 * t4234 / F::cast_from(144.0_f64) + t4265 * t4285 / F::cast_from(216.0_f64) + t9607 * t15869 / F::cast_from(3072.0_f64) - t12480 + t12530 - t12537 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1125 * t15873 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1125 * t15877;
    t15880
}
