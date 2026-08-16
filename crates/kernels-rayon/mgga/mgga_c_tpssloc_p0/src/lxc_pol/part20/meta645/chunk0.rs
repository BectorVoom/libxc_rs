//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2368/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368(t10216: f64, t13797: f64, t3067: f64, t353: f64, t373: f64, t383: f64, t1021: f64, t820: f64, t10482: f64, t1615: f64, t10390: f64, t10858: f64, t10883: f64, t13975: f64, t14069: f64, t14080: f64, t14211: f64, t2986: f64, t3039: f64, t3041: f64, t3057: f64, t3064: f64, t3121: f64, t42388: f64, t42397: f64, t42436: f64, t42460: f64, t42511: f64, t43235: f64, t43361: f64, t4575: f64, t4582: f64, t4593: f64, t45971: f64, t48265: f64) -> (f64, f64, f64, f64) {
    let t48585 = t13797 * t10216;
    let t48607 = t353 * t383 * t3067 * t373;
    let t48611 = t820 * t1021;
    let t48612 = t1615 * t10482;
    let t48622 = t42436 / 1152.0_f64 + t10390 * t14069 / 768.0_f64 + t42511 * t4575 / 1536.0_f64 + 7.0_f64 / 216.0_f64 * t2986 * t48585 * t45971 - t14080 * t3057 / 288.0_f64 - 5.0_f64 / 864.0_f64 * t14080 * t3064 - t3039 * t4582 * t13975 * t3121 / 1024.0_f64 + t10883 * t4582 * t13975 * t3041 / 1024.0_f64 - t3039 * t4582 * t4593 * t10858 / 3072.0_f64 + 5.0_f64 / 1728.0_f64 * t48607 * t42397 * t48265 + 3.0_f64 / 512.0_f64 * t42388 * t48611 * t48612 * t43235 - 3.0_f64 / 512.0_f64 * t43361 * t48611 * t14211 * t43235 + t42460 / 54.0_f64;
    (t48607, t48611, t48612, t48622)
}
