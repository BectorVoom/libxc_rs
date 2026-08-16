//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2364/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2364(t14488: f64, t376: f64, t1023: f64, t10408: f64, t1041: f64, t10413: f64, t14107: f64, t14220: f64, t14222: f64, t3039: f64, t3070: f64, t3071: f64, t42322: f64, t42324: f64, t42354: f64, t42369: f64, t42372: f64, t42546: f64, t43211: f64, t4337: f64, t4342: f64, t4582: f64, t4588: f64, t45993: f64, t48472: f64, t48477: f64, t48496: f64, t48497: f64) -> (f64, f64) {
    let t48506 = t376 * t14488;
    let t48511 = -t3070 * t3071 * t4342 * t48472 / 768.0_f64 - t10413 * t3071 * t48477 * t14220 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t4337 * t48472 - t42546 * t14222 / 768.0_f64 - t43211 * t14107 / 192.0_f64 + t42322 / 6912.0_f64 + 5.0_f64 / 6912.0_f64 * t42324 + 5.0_f64 / 13824.0_f64 * t1041 * t4582 * t4588 * t45993 + 55.0_f64 / 15552.0_f64 * t1041 * t4582 * t48496 * t48497 - t42369 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t42372 + t42354 * t14107 / 1024.0_f64 - t3039 * t4582 * t48506 * t1023 / 1024.0_f64;
    (t48506, t48511)
}
