//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 992/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk992(t10667: f64, t2389: f64, t774: f64, t1364: f64, t2116: f64, t8162: f64, t2169: f64, t3667: f64, t1381: f64, t8286: f64, t10470: f64, t10471: f64, t10472: f64, t10500: f64, t10501: f64, t7929: f64, t7932: f64, t7936: f64, t7945: f64, t8000: f64, t8001: f64, t8019: f64, t8023: f64, t8029: f64, t8040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10669 = t2389 * t774 * t10667;
    let t10672 = t1364 * t2116;
    let t10674 = t8162 * t774 * t10672;
    let t10678 = 7.0_f64 / 2304.0_f64 * t2169 * t3667;
    let t10679 = t8286 * t1381;
    let t10681 = t8000 + t8001 - t10470 - t8019 + t8023 + t10471 - t8029 - t10472 - t8040 + t10500 + t10501 + t7929 - t7932 - t7936 + t7945;
    (t10669, t10672, t10674, t10678, t10679, t10681)
}
