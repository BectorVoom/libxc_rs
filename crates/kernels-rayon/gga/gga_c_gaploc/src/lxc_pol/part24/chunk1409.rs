//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1409/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1409(t3394: f64, t493: f64, t6576: f64, t6578: f64, t1339: f64, t6583: f64, t31065: f64, t10144: f64, t1572: f64, t4673: f64, t1436: f64, t31040: f64, t31044: f64, t31046: f64, t31050: f64, t31053: f64, t31056: f64, t31068: f64, t34874: f64, t34877: f64, t34879: f64, t34881: f64, t34882: f64, t590: f64) -> f64 {
    let t34886 = t493 * t3394;
    let t34888 = t6576 * t34886 * t6578;
    let t34889 = 0.76685851907841499352e0_f64 * t34888;
    let t34890 = t1339 * t3394;
    let t34892 = t6583 * t34890 * t6578;
    let t34893 = 0.19171462976960374838e1_f64 * t34892;
    let t34894 = 0.31952438294933958064e-1_f64 * t31065;
    let t34897 = 0.95334639871601137784e0_f64 * t1572 * t4673 * t10144;
    let t34898 = t31040 + t31044 - t31046 - t31050 + t31053 - t31056 - t34874 + t34877 + t34879 + t34881 - 0.1022478025437886658e1_f64 * t1436 * t34882 * t590 + t34889 - t34893 + t34894 - t31068 + t34897;
    t34898
}
