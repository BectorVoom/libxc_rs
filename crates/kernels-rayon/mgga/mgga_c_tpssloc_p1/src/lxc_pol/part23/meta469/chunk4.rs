//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1390/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1390(t10403: f64, t10408: f64, t1041: f64, t10413: f64, t10876: f64, t1539: f64, t1616: f64, t17712: f64, t21118: f64, t21391: f64, t3070: f64, t3071: f64, t42309: f64, t42388: f64, t42624: f64, t4342: f64, t4582: f64, t4583: f64, t5398: f64, t5681: f64, t5685: f64, t5873: f64, t5878: f64, t5909: f64, t61950: f64, t62360: f64, t70640: f64, t70655: f64, t70660: f64, t70665: f64, t70703: f64, t75836: f64, t75912: f64, t77621: f64, t973: f64, t974: f64, t998: f64) -> f64 {
    let t77687 = -t1041 * t4582 * t4583 * t77621 / 576.0_f64 - 7.0_f64 / 54.0_f64 * t973 * t974 * t42624 * t75836 + t973 * t974 * t998 * t75912 / 288.0_f64 + 35.0_f64 / 972.0_f64 * t973 * t974 * t42309 * t75836 - t62360 / 2304.0_f64 - t10413 * t3071 * t5878 * t5685 / 768.0_f64 - t10403 * t3071 * t5681 * t5873 / 192.0_f64 + t61950 * t5909 / 384.0_f64 + t42388 * t3071 * t21391 * t1539 / 192.0_f64 - 3.0_f64 / 256.0_f64 * t10876 * t4582 * t17712 * t5873 - t70640 / 288.0_f64 - t70655 / 27.0_f64 + t70660 / 216.0_f64 + 7.0_f64 / 486.0_f64 * t70665 - t3070 * t3071 * t4342 * t1616 * t5398 / 192.0_f64 - 5.0_f64 / 576.0_f64 * t3070 * t10408 * t21118 * t1616 + t70703 / 288.0_f64;
    t77687
}
