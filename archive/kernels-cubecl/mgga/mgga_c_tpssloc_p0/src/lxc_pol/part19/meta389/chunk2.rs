//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1464/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464<F: Float>(t11655: F, t11731: F, t11738: F, t1174: F, t11825: F, t1214: F, t1227: F, t15615: F, t15654: F, t248: F, t3490: F, t3494: F, t3555: F, t3587: F, t39097: F, t39103: F, t42468: F, t43764: F, t44699: F, t44725: F, t44803: F, t44805: F, t44811: F, t44817: F, t44828: F, t44836: F, t44847: F, t44851: F, t44858: F, t44863: F, t44871: F, t44873: F, t4582: F, t475: F, t974: F) -> F {
    let t44878 = -F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t44803 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1174 * t974 * t44805 * t39097 + t44811 / F::cast_from(216.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1227 * t4582 * t15654 * t42468 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1174 * t974 * t44817 * t39097 + F::cast_from(5.0_f64) / F::cast_from(576.0_f64) * t3490 * t11655 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t11825 * t3587 + F::cast_from(55.0_f64) / F::cast_from(15552.0_f64) * t1227 * t248 * t44828 * t43764 - t44836 * t248 * t1214 * t44699 * t475 / F::cast_from(3072.0_f64) - t1174 * t974 * t3555 * t39103 / F::cast_from(48.0_f64) - t44847 / F::cast_from(162.0_f64) + t44851 / F::cast_from(1152.0_f64) - t1227 * t4582 * t15615 * t42468 / F::cast_from(128.0_f64) - t44858 * t11731 / F::cast_from(128.0_f64) + t44863 * t248 * t1214 * t44699 * t44725 / F::cast_from(128.0_f64) + t44871 / F::cast_from(192.0_f64) + t11738 * t4582 * t44873 * t3494 / F::cast_from(512.0_f64);
    t44878
}
