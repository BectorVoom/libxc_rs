//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2459/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459(t2770: f64, t2987: f64, t10277: f64, t4509: f64, t10390: f64, t13765: f64, t10937: f64, t14501: f64, t10408: f64, t10915: f64, t13554: f64, t14033: f64, t14037: f64, t2986: f64, t3070: f64, t42496: f64, t43303: f64, t43307: f64, t43310: f64, t43313: f64, t4575: f64, t45971: f64, t4644: f64, t49976: f64) -> f64 {
    let t50366 = t2987 * t2770;
    let t50370 = t4509 * t10277;
    let t50378 = t10390 * t13765;
    let t50384 = t10937 * t14501;
    let t50393 = t2986 * t50366 * t45971 / 16.0_f64 - t2986 * t50370 * t45971 / 12.0_f64 + 19.0_f64 / 864.0_f64 * t43303 - t43307 - 77.0_f64 / 486.0_f64 * t43310 - t42496 * t4575 / 144.0_f64 + t50378 / 1152.0_f64 + t10390 * t14033 / 1536.0_f64 + 5.0_f64 / 4608.0_f64 * t10390 * t14037 - t50384 / 216.0_f64 - t4644 * t10915 / 768.0_f64 - t43313 / 108.0_f64 - 5.0_f64 / 768.0_f64 * t3070 * t10408 * t13554 * t49976;
    t50393
}
