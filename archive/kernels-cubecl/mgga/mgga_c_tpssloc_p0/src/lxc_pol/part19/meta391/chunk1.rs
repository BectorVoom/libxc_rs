//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1472/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472<F: Float>(t3584: F, t676: F, t1227: F, t248: F, t3243: F, t11159: F, t11665: F, t11668: F, t11678: F, t11684: F, t11721: F, t1174: F, t1177: F, t11805: F, t1214: F, t1216: F, t15620: F, t15661: F, t15708: F, t2250: F, t3247: F, t3490: F, t3508: F, t3577: F, t3578: F, t42374: F, t43723: F, t44699: F, t45002: F, t45007: F, t45009: F, t45013: F, t45015: F, t45020: F, t45027: F, t45030: F, t45037: F, t45044: F, t4582: F, t4987: F) -> F {
    let t45046 = t676 * t3584;
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45066 = -t11665 * t11684 / F::cast_from(384.0_f64) + t45002 / F::cast_from(2592.0_f64) - t1174 * t1177 * t43723 / F::cast_from(36.0_f64) + t45007 / F::cast_from(1152.0_f64) - t45009 / F::cast_from(576.0_f64) - t45013 / F::cast_from(1728.0_f64) - t45015 / F::cast_from(288.0_f64) + t45020 / F::cast_from(2592.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t1227 * t4582 * t4987 * t42374 - t45027 / F::cast_from(288.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t45030 * t248 * t1214 * t44699 * t11721 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t45037 * t248 * t1214 * t44699 * t3508 - F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t45044 - F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t45049 - t3577 * t3578 * t3247 * t2250 * t15708 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t3577 * t11668 * t1216 * t11159 - t11678 * t3578 * t15620 * t15661 / F::cast_from(192.0_f64) - t3490 * t11805 / F::cast_from(1152.0_f64);
    t45066
}
