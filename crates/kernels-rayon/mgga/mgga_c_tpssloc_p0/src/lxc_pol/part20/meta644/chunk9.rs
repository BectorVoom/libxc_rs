//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2367/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2367(t13969: f64, t13976: f64, t3130: f64, t1041: f64, t14183: f64, t10471: f64, t47840: f64, t10479: f64, t10908: f64, t4641: f64, t10485: f64, t10937: f64, t10965: f64, t14033: f64, t14037: f64, t14164: f64, t2979: f64, t42428: f64, t42432: f64, t4582: f64, t4585: f64, t4590: f64, t47697: f64, t48548: f64, t48554: f64, t973: f64) -> (f64, f64) {
    let t48564 = t3130 * t13969 * t13976;
    let t48567 = t1041 * t13969 * t14183;
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48574 = t4641 * t10908;
    let t48577 = t973 * t2979 * t47697 / 216.0_f64 + 5.0_f64 / 6912.0_f64 * t48548 - t10937 * t14033 / 288.0_f64 - 5.0_f64 / 864.0_f64 * t10937 * t14037 + t1041 * t4582 * t14164 * t48554 / 256.0_f64 - t10965 * t4585 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t10965 * t4590 + t48564 / 384.0_f64 + 5.0_f64 / 6912.0_f64 * t48567 + t48570 * t10485 / 512.0_f64 + 19.0_f64 / 864.0_f64 * t42428 + t48574 / 1536.0_f64 - t42432 / 6912.0_f64;
    (t48569, t48577)
}
