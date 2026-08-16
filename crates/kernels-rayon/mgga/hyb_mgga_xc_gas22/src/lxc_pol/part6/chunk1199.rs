//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1199/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1199(t22195: f64, t458: f64, t496: f64, t2750: f64, t457: f64, t2747: f64, t2751: f64, t2754: f64, t2757: f64, t1101: f64, t7544: f64, t1074: f64, t1082: f64, t1089: f64, t1097: f64, t1884: f64, t21836: f64, t21840: f64, t21856: f64, t21874: f64, t21894: f64, t21940: f64, t21969: f64, t21973: f64, t221: f64, t222: f64, t2647: f64, t2771: f64, t2772: f64, t2774: f64, t2803: f64, t3021: f64, t479: f64, t567: f64, t7253: f64, t7330: f64, t7359: f64, t7364: f64, t7374: f64, t7385: f64, t7399: f64, t7476: f64, t7478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22199 = 840.0_f64 * t458 / t22195 * t496;
    let t22204 = t457 * t2750 * t496;
    let t22208 = t2751 * t2747;
    let t22210 = t2754 * t2747;
    let t22212 = t2757 * t2747;
    let t22215 = 480.0_f64 * t7544 * t1101;
    let t22252 = t21836 + t21840 + t21856 - t21894 - t21940 - 8.0_f64 * t2772 * t1082 * t7359 - 0.55209406483950617283e-2_f64 * t221 * t21874 * t479 - 0.21309037037037037036e0_f64 * t222 * t3021 * t1074 * t1082 - 0.27397333333333333333e0_f64 * t222 * t1884 * t2771 * t2774 + 0.13218100589565368422e2_f64 * t222 * t567 * t7476 * t7478 - 0.67471172535210825684e-1_f64 * t222 * t3021 * t1089 * t1097 - 0.86748650402413918736e-1_f64 * t222 * t1884 * t2647 * t2803 - t21969 - t21973 - 0.1301229756036208781e0_f64 * t222 * t7374 * t7330 - 0.41096e0_f64 * t222 * t7385 * t7364 + 0.38527786510141256862e1_f64 * t222 * t567 * t7253 * t7399;
    (t22199, t22204, t22208, t22210, t22212, t22215, t22252)
}
