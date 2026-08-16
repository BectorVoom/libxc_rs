//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1200/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1200(t6703: f64, t7593: f64, t1920: f64, t32923: f64, t968: f64, t1945: f64, t7577: f64, t10165: f64, t1052: f64, t113217: f64, t113231: f64, t113240: f64, t14529: f64, t14552: f64, t1635: f64, t1927: f64, t23327: f64, t23346: f64, t23369: f64, t25442: f64, t25755: f64, t3026: f64, t30793: f64, t30904: f64, t32913: f64, t32976: f64, t32998: f64, t4557: f64, t4693: f64, t6687: f64, t6691: f64, t6706: f64, t6816: f64, t7625: f64, t8396: f64, t8397: f64, t8407: f64, t986: f64) -> f64 {
    let t119076 = t6703 * t7593;
    let t119086 = t1920 * t968 * t32923;
    let t119088 = t7577 * t1945;
    let t119107 = -0.54831135561607547883e-2_f64 * t113217 - t14529 * t8407 - 2.0_f64 * t25755 * t6816 - 0.16449340668482264365e-1_f64 * t6687 * t986 * t32923 + 2.0_f64 * t14552 * t8397 - 0.16449340668482264365e-1_f64 * t6687 * t119076 * t6706 - 0.18277045187202515961e-2_f64 * t113240 + 0.43864908449286038307e-1_f64 * t23346 * t32998 + 2.0_f64 * t3026 * t32913 + 0.54831135561607547883e-2_f64 * t119086 - 0.54831135561607547883e-2_f64 * t23327 * t119088 * t6691 - t14552 * t8407 + 4.0_f64 * t4557 * t30793 - 2.0_f64 * t23369 * t7625 - t113231 * t1635 - 6.0_f64 * t1052 * t10165 * t8396 * t4693 + 0.43864908449286038307e-1_f64 * t23346 * t32976 - 0.3289868133696452873e-1_f64 * t1927 * t25442 * t30904;
    t119107
}
