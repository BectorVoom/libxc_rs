//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2593/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593<F: Float>(t11728: F, t22312: F, t248: F, t3570: F, t1174: F, t1177: F, t15495: F, t6221: F, t65552: F, t65554: F, t65558: F, t65567: F, t71189: F, t71201: F, t72273: F, t72285: F, t72287: F, t72289: F, t72293: F) -> F {
    let t72297 = t11728 * t248 * t3570 * t22312;
    let t72299 = t65552 / F::cast_from(3456.0_f64) + t65554 / F::cast_from(1536.0_f64) - t72273 / F::cast_from(6912.0_f64) - t65558 / F::cast_from(2304.0_f64) + t65567 / F::cast_from(36.0_f64) - t1174 * t1177 * t71201 / F::cast_from(48.0_f64) - t1174 * t1177 * t71189 / F::cast_from(48.0_f64) - t15495 * t6221 / F::cast_from(192.0_f64) - t72285 / F::cast_from(1152.0_f64) + t72287 / F::cast_from(768.0_f64) + t72289 / F::cast_from(432.0_f64) + t72293 / F::cast_from(4608.0_f64) - t72297 / F::cast_from(768.0_f64);
    t72299
}
