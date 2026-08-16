//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2980/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980(t1041: f64, t13969: f64, t17696: f64, t1021: f64, t10390: f64, t10413: f64, t14211: f64, t17681: f64, t17688: f64, t17925: f64, t17976: f64, t17991: f64, t248: f64, t2780: f64, t2960: f64, t2986: f64, t3039: f64, t3071: f64, t3117: f64, t360: f64, t42546: f64, t42610: f64, t42613: f64, t43361: f64, t48477: f64, t48611: f64, t49757: f64, t50366: f64, t55677: f64, t55716: f64, t5878: f64, t59659: f64, t61719: f64, t973: f64, t974: f64, t977: f64, t998: f64) -> f64 {
    let t62210 = t1041 * t13969 * t17696;
    let t62225 = -t3039 * t248 * t1021 * t61719 * t360 / 1536.0_f64 + t10390 * t17681 / 2304.0_f64 + t973 * t974 * t998 * t55677 / 288.0_f64 - t42610 / 1296.0_f64 - t42613 / 972.0_f64 + t49757 / 2304.0_f64 - t10413 * t3071 * t5878 * t2780 / 4608.0_f64 - t973 * t977 * t59659 / 12.0_f64 + 2.0_f64 / 27.0_f64 * t2960 * t17991 + 5.0_f64 / 3888.0_f64 * t62210 - t3117 * t17976 / 576.0_f64 - 5.0_f64 / 1152.0_f64 * t3117 * t17688 + t2986 * t50366 * t55716 / 12.0_f64 - t42546 * t17925 / 1152.0_f64 - t43361 * t48611 * t14211 * t48477 / 128.0_f64;
    t62225
}
