//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3007/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007(t5893: f64, t698: f64, t973: f64, t17615: f64, t2960: f64, t3131: f64, t5866: f64, t1022: f64, t5872: f64, t10263: f64, t10403: f64, t10413: f64, t13995: f64, t14213: f64, t14215: f64, t14220: f64, t14228: f64, t14230: f64, t3070: f64, t3071: f64, t42483: f64, t43352: f64, t43354: f64, t4342: f64, t4575: f64, t49929: f64, t50324: f64, t50425: f64, t50429: f64, t5677: f64, t5894: f64, t61775: f64) -> f64 {
    let t62832 = t973 * t698 * t5893;
    let t62836 = t2960 * t17615;
    let t62840 = t5866 * t3131;
    let t62845 = t5866 * t1022;
    let t62850 = t5872 * t1022;
    let t62871 = -t62832 / 972.0_f64 + 11.0_f64 / 243.0_f64 * t10263 * t5894 - t62836 / 162.0_f64 - t13995 * t14230 / 576.0_f64 + t10403 * t3071 * t62840 * t14213 / 1152.0_f64 - t10413 * t3071 * t62845 * t14220 / 2304.0_f64 + t42483 * t3071 * t62850 * t14220 / 2304.0_f64 + t3070 * t3071 * t5677 * t14228 / 384.0_f64 - t3070 * t3071 * t4342 * t61775 / 576.0_f64 - t43352 / 13824.0_f64 - 19.0_f64 / 7776.0_f64 * t43354 + t50324 * t4575 / 1152.0_f64 + 5.0_f64 / 1944.0_f64 * t50425 + t50429 / 3456.0_f64 + t49929 * t14215 / 576.0_f64;
    t62871
}
