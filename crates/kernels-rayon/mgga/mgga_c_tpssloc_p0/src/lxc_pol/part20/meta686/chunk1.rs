//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2600/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600(t1174: f64, t14726: f64, t44562: f64, t3577: f64, t44951: f64, t4953: f64, t11677: f64, t15245: f64, t11665: f64, t11668: f64, t11670: f64, t11694: f64, t1177: f64, t11853: f64, t1227: f64, t1230: f64, t15569: f64, t15714: f64, t248: f64, t3243: f64, t3515: f64, t44851: f64, t44871: f64, t4582: f64, t4977: f64, t5012: f64, t50830: f64, t50929: f64) -> f64 {
    let t52751 = t1174 * t44562 * t14726;
    let t52758 = t3577 * t44951 * t4953;
    let t52759 = t52758 / 6912.0_f64;
    let t52766 = t15245 * t11677;
    let t52769 = -t1227 * t248 * t1230 * t50830 / 4608.0_f64 + t44851 / 4608.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t5012 * t3243 + 5.0_f64 / 4608.0_f64 * t11665 * t15714 - 7.0_f64 / 648.0_f64 * t52751 - t1174 * t1177 * t50929 / 48.0_f64 + t44871 / 768.0_f64 + t52759 - 5.0_f64 / 864.0_f64 * t15569 * t11670 - t3515 * t4582 * t4977 * t11853 / 3072.0_f64 + t52766 * t11694 / 1536.0_f64;
    t52769
}
