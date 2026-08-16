//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1475/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1475<F: Float>(t1090: F, t11148: F, t11163: F, t11172: F, t11665: F, t11670: F, t11729: F, t11739: F, t11809: F, t11825: F, t11853: F, t1216: F, t1218: F, t1227: F, t1230: F, t248: F, t3490: F, t3531: F, t3577: F, t3578: F, t43800: F, t43804: F, t45080: F, t45086: F, t45102: F, t45108: F, t45112: F, t45114: F, t45119: F, t45126: F, t45128: F) -> F {
    let t45133 = -t1227 * t248 * t1230 * t43804 / F::cast_from(4608.0_f64) - t3490 * t11809 / F::cast_from(192.0_f64) - t1227 * t248 * t1230 * t43800 / F::cast_from(192.0_f64) + t45080 * t1218 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t11665 * t11670 + t45086 / F::cast_from(576.0_f64) - t3577 * t3578 * t11172 * t1216 / F::cast_from(1152.0_f64) - t3577 * t3578 * t11163 * t1216 / F::cast_from(192.0_f64) - t3577 * t3578 * t11853 * t1090 / F::cast_from(1152.0_f64) + t45102 / F::cast_from(1152.0_f64) - t11825 * t3531 / F::cast_from(384.0_f64) - t45108 / F::cast_from(288.0_f64) - t45112 + t45114 * t3578 * t11729 * t1090 / F::cast_from(192.0_f64) - t45119 * t3578 * t11739 * t1090 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t45126 - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t3577 * t45128 * t11148 * t1216;
    t45133
}
