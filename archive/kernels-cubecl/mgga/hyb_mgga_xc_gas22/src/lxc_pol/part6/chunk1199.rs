//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1199/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1199<F: Float>(t22195: F, t458: F, t496: F, t2750: F, t457: F, t2747: F, t2751: F, t2754: F, t2757: F, t1101: F, t7544: F, t1074: F, t1082: F, t1089: F, t1097: F, t1884: F, t21836: F, t21840: F, t21856: F, t21874: F, t21894: F, t21940: F, t21969: F, t21973: F, t221: F, t222: F, t2647: F, t2771: F, t2772: F, t2774: F, t2803: F, t3021: F, t479: F, t567: F, t7253: F, t7330: F, t7359: F, t7364: F, t7374: F, t7385: F, t7399: F, t7476: F, t7478: F) -> (F, F, F, F, F, F, F) {
    let t22199 = F::cast_from(840.0_f64) * t458 / t22195 * t496;
    let t22204 = t457 * t2750 * t496;
    let t22208 = t2751 * t2747;
    let t22210 = t2754 * t2747;
    let t22212 = t2757 * t2747;
    let t22215 = F::cast_from(480.0_f64) * t7544 * t1101;
    let t22252 = t21836 + t21840 + t21856 - t21894 - t21940 - F::cast_from(8.0_f64) * t2772 * t1082 * t7359 - F::cast_from(0.55209406483950617283e-2_f64) * t221 * t21874 * t479 - F::cast_from(0.21309037037037037036e0_f64) * t222 * t3021 * t1074 * t1082 - F::cast_from(0.27397333333333333333e0_f64) * t222 * t1884 * t2771 * t2774 + F::cast_from(0.13218100589565368422e2_f64) * t222 * t567 * t7476 * t7478 - F::cast_from(0.67471172535210825684e-1_f64) * t222 * t3021 * t1089 * t1097 - F::cast_from(0.86748650402413918736e-1_f64) * t222 * t1884 * t2647 * t2803 - t21969 - t21973 - F::cast_from(0.1301229756036208781e0_f64) * t222 * t7374 * t7330 - F::cast_from(0.41096e0_f64) * t222 * t7385 * t7364 + F::cast_from(0.38527786510141256862e1_f64) * t222 * t567 * t7253 * t7399;
    (t22199, t22204, t22208, t22210, t22212, t22215, t22252)
}
