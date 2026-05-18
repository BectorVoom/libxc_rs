//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1381/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1381<F: Float>(t14677: F, t92581: F, t33848: F, t8064: F, t1291: F, t15692: F, t15788: F, t1872: F, t27134: F, t27144: F, t3669: F, t437: F, t7809: F, t96732: F, t96776: F, t96802: F, t96823: F, t96846: F, t96869: F, t96890: F, t96920: F, t96948: F, t96971: F, t97006: F, t97033: F, t97066: F, t97098: F, t97125: F, t97147: F, t97170: F, t97195: F, t97215: F, t97232: F, t97263: F, t97282: F, t97303: F, t97319: F, t97347: F, t97371: F, t97393: F, t97411: F, t97434: F, t97454: F, t97470: F, t97487: F, t97494: F, t97499: F, t97500: F, t97501: F, t97503: F, t97505: F, t97507: F, t97510: F, t97511: F, t97513: F, t97517: F, t97521: F) -> (F, F, F) {
    let t97526 = F::new(6.0) * t92581 * t14677;
    let t97528 = F::new(2.0) * t33848 * t8064;
    let t97529 = (t96776 + t96732 + t97487 + t97470 + t97454 + t96802 + t97434 + t97411 + t97393 + t97371 + t97347 + t97319 + t97303 + t97282 + t97263 + t97232 + t97215 + t97195 + t97170 + t97147 + t97125 + t97098 + t97066 + t97033 + t97006 + t96971 + t96948 + t96869 + t96920 + t96890 + t96823 + t96846) * t437 - F::new(2.0) * t97494 * t1291 + t97499 + t97500 + t97501 + t97503 - t97505 - t97507 - t97510 + t97511 + t97513 + F::new(2.0) * t15692 * t27144 - t97517 - t7809 * t15788 - t97521 + F::new(2.0) * t3669 * t27134 * t1872 + t97526 - t97528;
    (t97526, t97528, t97529)
}
