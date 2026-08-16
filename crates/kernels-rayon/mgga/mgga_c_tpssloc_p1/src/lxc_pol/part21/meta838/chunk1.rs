//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2991/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2991(t10422: f64, t17676: f64, t3070: f64, t225: f64, t61618: f64, t10413: f64, t14122: f64, t14126: f64, t14489: f64, t1539: f64, t1616: f64, t2979: f64, t3071: f64, t369: f64, t378: f64, t4343: f64, t4650: f64, t49929: f64, t49934: f64, t50132: f64, t50147: f64, t50169: f64, t50172: f64, t50174: f64, t50181: f64, t59715: f64, t59767: f64, t61871: f64, t68: f64, t973: f64, t977: f64) -> (f64, f64) {
    let t62602 = t3070 * t10422 * t17676;
    let t62604 = t61618 * t225;
    let t62616 = t50132 / 432.0_f64 + t49929 * t14122 / 1152.0_f64 - t50147 / 5184.0_f64 - t49934 * t14126 / 2304.0_f64 - t10413 * t3071 * t1616 * t61871 / 1152.0_f64 - t3070 * t3071 * t4650 * t4343 / 576.0_f64 - t973 * t977 * t59767 / 72.0_f64 - t973 * t2979 * t59715 / 36.0_f64 - t50169 / 5184.0_f64 + t62602 / 1728.0_f64 + t62604 * t68 * t369 * t378 / 3072.0_f64 - t50172 / 81.0_f64 + t3070 * t3071 * t14489 * t1539 / 2304.0_f64 - t50174 / 3456.0_f64 + t50181 / 5184.0_f64;
    (t62604, t62616)
}
