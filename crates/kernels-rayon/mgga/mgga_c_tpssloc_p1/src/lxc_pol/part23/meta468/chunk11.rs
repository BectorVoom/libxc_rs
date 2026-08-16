//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1385/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385(t76995: f64, t77017: f64, t77151: f64, t77483: f64, t1020: f64, t1021: f64, t1041: f64, t1044: f64, t1618: f64, t17607: f64, t21580: f64, t248: f64, t3062: f64, t3131: f64, t360: f64, t369: f64, t378: f64, t42347: f64, t43317: f64, t4644: f64, t5880: f64, t5900: f64, t61739: f64, t68: f64, t70148: f64, t70162: f64, t70166: f64, t70199: f64, t70209: f64, t70214: f64, t70227: f64, t75836: f64, t76597: f64, t76612: f64, t76620: f64, t76740: f64, t76977: f64, t973: f64, t974: f64) -> (f64, f64) {
    let t77485 = t76995 + t77017 + t77151 + t77483;
    let t77498 = -t70162 / 192.0_f64 + t70166 / 288.0_f64 - 5.0_f64 / 576.0_f64 * t4644 * t21580 + t70148 * t1618 / 768.0_f64 - t17607 * t5900 / 384.0_f64 + t70199 / 1728.0_f64 - t1041 * t248 * t1044 * t76612 / 192.0_f64 + t70209 / 192.0_f64 + t70214 / 384.0_f64 + t76977 * t68 * t369 * t378 / 3072.0_f64 - t1041 * t248 * t1044 * t76620 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t1041 * t248 * t3062 * t76597 + t70227 / 192.0_f64 + t973 * t974 * t43317 * t75836 / 6.0_f64 + t1020 * t248 * t1021 * t77485 * t360 / 3072.0_f64 + 7.0_f64 / 1536.0_f64 * t42347 * t248 * t1021 * t76740 * t3131 - t61739 * t5880 / 512.0_f64;
    (t77485, t77498)
}
