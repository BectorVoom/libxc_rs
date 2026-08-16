//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2768/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768(t13084: f64, t16836: f64, t16839: f64, t16891: f64, t16896: f64, t16898: f64, t16901: f64, t2633: f64, t2643: f64, t2645: f64, t2679: f64, t2684: f64, t40966: f64, t40982: f64, t40990: f64, t4178: f64, t4180: f64, t4240: f64, t47044: f64, t58353: f64, t58363: f64, t58373: f64, t58379: f64, t58381: f64, t9642: f64, t9646: f64, t9647: f64) -> f64 {
    let t58392 = -t4178 * t2645 * t16901 * t2633 / 384.0_f64 + 7.0_f64 / 384.0_f64 * t58353 - 5.0_f64 / 768.0_f64 * t2643 * t9646 * t16891 * t9647 - t2643 * t4180 * t16839 * t2684 / 3072.0_f64 + 7.0_f64 / 2304.0_f64 * t58363 - t16836 * t13084 / 192.0_f64 + 7.0_f64 / 1536.0_f64 * t4178 * t4180 * t16839 * t2633 - t47044 * t4240 / 768.0_f64 - 7.0_f64 / 288.0_f64 * t58373 - 5.0_f64 / 768.0_f64 * t2643 * t9646 * t16839 * t9647 - 7.0_f64 / 288.0_f64 * t58379 + 7.0_f64 / 1152.0_f64 * t58381 - 5.0_f64 / 384.0_f64 * t9642 * t16898 - 5.0_f64 / 768.0_f64 * t2643 * t9646 * t16896 * t2679 + 595.0_f64 / 1296.0_f64 * t40966 - 119.0_f64 / 3456.0_f64 * t40982 + 595.0_f64 / 3456.0_f64 * t40990;
    t58392
}
