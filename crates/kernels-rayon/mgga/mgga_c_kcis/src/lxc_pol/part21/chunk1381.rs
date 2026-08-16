//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1381/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1381(t14677: f64, t92581: f64, t33848: f64, t8064: f64, t1291: f64, t15692: f64, t15788: f64, t1872: f64, t27134: f64, t27144: f64, t3669: f64, t437: f64, t7809: f64, t96732: f64, t96776: f64, t96802: f64, t96823: f64, t96846: f64, t96869: f64, t96890: f64, t96920: f64, t96948: f64, t96971: f64, t97006: f64, t97033: f64, t97066: f64, t97098: f64, t97125: f64, t97147: f64, t97170: f64, t97195: f64, t97215: f64, t97232: f64, t97263: f64, t97282: f64, t97303: f64, t97319: f64, t97347: f64, t97371: f64, t97393: f64, t97411: f64, t97434: f64, t97454: f64, t97470: f64, t97487: f64, t97494: f64, t97499: f64, t97500: f64, t97501: f64, t97503: f64, t97505: f64, t97507: f64, t97510: f64, t97511: f64, t97513: f64, t97517: f64, t97521: f64) -> (f64, f64, f64) {
    let t97526 = 6.0_f64 * t92581 * t14677;
    let t97528 = 2.0_f64 * t33848 * t8064;
    let t97529 = (t96776 + t96732 + t97487 + t97470 + t97454 + t96802 + t97434 + t97411 + t97393 + t97371 + t97347 + t97319 + t97303 + t97282 + t97263 + t97232 + t97215 + t97195 + t97170 + t97147 + t97125 + t97098 + t97066 + t97033 + t97006 + t96971 + t96948 + t96869 + t96920 + t96890 + t96823 + t96846) * t437 - 2.0_f64 * t97494 * t1291 + t97499 + t97500 + t97501 + t97503 - t97505 - t97507 - t97510 + t97511 + t97513 + 2.0_f64 * t15692 * t27144 - t97517 - t7809 * t15788 - t97521 + 2.0_f64 * t3669 * t27134 * t1872 + t97526 - t97528;
    (t97526, t97528, t97529)
}
