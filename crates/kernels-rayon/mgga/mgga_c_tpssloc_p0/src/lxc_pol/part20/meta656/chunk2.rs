//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2426/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426(t3041: f64, t607: f64, t1023: f64, t3120: f64, t10390: f64, t14501: f64, t10422: f64, t13761: f64, t3070: f64, t1020: f64, t1021: f64, t1031: f64, t10413: f64, t13941: f64, t14093: f64, t1539: f64, t248: f64, t3071: f64, t3088: f64, t3117: f64, t360: f64, t378: f64, t42514: f64, t42518: f64, t4342: f64, t4347: f64, t4616: f64, t48670: f64, t48674: f64, t49588: f64) -> (f64, f64, f64) {
    let t49594 = t3041 * t607;
    let t49599 = t1023 * t3120;
    let t49604 = t10390 * t14501;
    let t49607 = t3070 * t10422 * t13761;
    let t49609 = -t10413 * t3071 * t4347 * t3041 / 1536.0_f64 - t42514 / 432.0_f64 - 5.0_f64 / 1296.0_f64 * t42518 + 19.0_f64 / 576.0_f64 * t4616 * t3088 * t378 - t13941 * t1031 * t378 / 192.0_f64 + t48670 / 10368.0_f64 + t48674 / 15552.0_f64 + t3117 * t14093 / 1536.0_f64 + t1020 * t248 * t1021 * t49588 * t360 / 3072.0_f64 + t10413 * t3071 * t4342 * t49594 / 768.0_f64 - t10413 * t3071 * t1539 * t49599 / 1536.0_f64 + t49604 / 1152.0_f64 + t49607 / 1152.0_f64;
    (t49594, t49599, t49609)
}
