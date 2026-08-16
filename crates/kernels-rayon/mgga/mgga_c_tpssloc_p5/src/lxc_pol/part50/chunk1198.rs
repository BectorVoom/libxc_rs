//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1198/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1198(t118429: f64, t118964: f64, t1054: f64, t7624: f64, t23384: f64, t32976: f64, t33001: f64, t113177: f64, t113236: f64, t14545: f64, t1927: f64, t1955: f64, t23327: f64, t23329: f64, t23336: f64, t23394: f64, t25742: f64, t25759: f64, t25814: f64, t2775: f64, t30781: f64, t30915: f64, t32924: f64, t32969: f64, t32970: f64, t32980: f64, t3961: f64, t4347: f64, t4694: f64, t6680: f64, t6687: f64, t6704: f64, t7553: f64, t7599: f64, t82402: f64, t82417: f64, t8407: f64, t88112: f64, t883: f64, t884: f64, t88772: f64) -> (f64, f64, f64) {
    let t118965 = t118429 + t118964;
    let t118971 = t1054 * t7624;
    let t119008 = t23384 * t32976;
    let t119010 = t23384 * t33001;
    let t119016 = -0.9869604401089358619e-1_f64 * t1927 * t23329 * t25759 + 0.14621636149762012769e-1_f64 * t82402 * t32970 - 0.54831135561607547883e-2_f64 * t23327 * t23329 * t118971 * t884 - 0.54831135561607547883e-2_f64 * t23327 * t23329 * t30781 * t4347 - 0.54831135561607547883e-2_f64 * t23327 * t113236 * t7553 + 0.10966227112321509577e-1_f64 * t23327 * t88772 * t7599 * t884 + 0.10966227112321509577e-1_f64 * t23327 * t88112 * t1955 * t2775 * t3961 + 0.10966227112321509577e-1_f64 * t23327 * t88772 * t1955 * t883 * t25814 - 0.3289868133696452873e-1_f64 * t1927 * t23336 * t32980 + 0.54831135561607547883e-2_f64 * t113177 - 0.43864908449286038307e-1_f64 * t6680 * t32924 - t14545 * t8407 + 0.3289868133696452873e-1_f64 * t6687 * t6704 * t23394 * t25742 - 0.54831135561607547883e-2_f64 * t119008 - 0.54831135561607547883e-2_f64 * t119010 - t30915 * t4694 - 0.54831135561607547883e-2_f64 * t23327 * t82417 * t32969;
    (t118965, t118971, t119016)
}
