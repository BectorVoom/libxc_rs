//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 965/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk965(t43454: f64, t11001: f64, t9823: f64, t33206: f64, t959: f64, t13096: f64, t1890: f64, t1966: f64, t40986: f64, t40989: f64, t43413: f64, t43414: f64, t43417: f64, t43421: f64, t43426: f64, t43433: f64, t43435: f64, t43440: f64, t43442: f64, t43444: f64, t43447: f64, t43448: f64, t43449: f64, t43450: f64, t590: f64) -> f64 {
    let t43455 = 0.17875244975925213335e0_f64 * t43454;
    let t43456 = t9823 * t11001;
    let t43458 = t33206 * t959;
    let t43460 = -t43413 + t43414 - t43417 + 0.38342925953920749676e0_f64 * t40986 - 0.11502877786176224903e1_f64 * t43421 + 0.72851559312449424384e1_f64 * t40989 - t43426 - 0.51123901271894332902e0_f64 * t1966 * t1890 * t13096 * t590 - t43433 - 0.76685851907841499352e0_f64 * t43435 + t43440 - 0.38342925953920749676e0_f64 * t43442 - t43444 - t43447 + t43448 - t43449 + t43450 - t43455 + 0.71500979903700853338e0_f64 * t43456 + 0.29792074959875355558e-1_f64 * t43458;
    t43460
}
