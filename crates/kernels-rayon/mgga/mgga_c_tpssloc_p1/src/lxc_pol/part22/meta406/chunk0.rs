//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1706/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1706(t18232: f64, t3297: f64, t136: f64, t1113: f64, t18237: f64, t18241: f64, t11211: f64, t11487: f64, t14766: f64, t15347: f64, t15348: f64, t15349: f64, t18494: f64, t18497: f64, t18500: f64, t18503: f64, t18505: f64, t18508: f64, t18510: f64, t18512: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18514 = t3297 * t18232;
    let t18515 = t136 * t18514;
    let t18517 = t1113 * t18237;
    let t18518 = t136 * t18517;
    let t18520 = t1113 * t18241;
    let t18521 = t136 * t18520;
    let t18523 = t11487 - 5.0_f64 / 27.0_f64 * t11211 - 10.0_f64 / 27.0_f64 * t14766 - t15347 + t15348 + t15349 - t18494 / 27.0_f64 - 2.0_f64 / 27.0_f64 * t18497 + t18500 / 3.0_f64 + t18503 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t18505 - t18508 - 2.0_f64 / 3.0_f64 * t18510 + t18512 / 9.0_f64 + t18515 / 18.0_f64 - t18518 / 3.0_f64 - t18521 / 6.0_f64;
    (t18514, t18515, t18517, t18518, t18520, t18521, t18523)
}
