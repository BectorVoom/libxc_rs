//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1438/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438(t11190: f64, t6020: f64, t6024: f64, t1670: f64, t21810: f64, t3264: f64, t3313: f64, t71701: f64, t11275: f64, t18265: f64, t6267: f64, t15376: f64, t15395: f64, t18409: f64, t18416: f64, t18427: f64, t18469: f64, t22063: f64, t22066: f64, t3447: f64, t4919: f64, t52100: f64, t64644: f64, t73188: f64, t73199: f64, t73225: f64, t73272: f64, t73496: f64, t78035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78364 = 0.57895126195293126241e3_f64 * t11190 * t6024 * t6020;
    let t78367 = 8.0_f64 * t3264 * t21810 * t1670;
    let t78370 = 0.64327917994770140268e2_f64 * t3313 * t71701 * t1670;
    let t78373 = 0.3103560775156404018e4_f64 * t11275 * t18265 * t6020;
    let t78379 = t6267 * t6267;
    let t78423 = -0.59259259259259259256e-2_f64 * t73188 + 0.22222222222222222221e-2_f64 * t73199 + 0.66666666666666666664e-2_f64 * t3447 * t4919 * t73225 - 0.22222222222222222222e-2_f64 * t3447 * t64644 * t18469 + 0.16666666666666666666e-2_f64 * t3447 * t18416 * t18409 + 0.33333333333333333332e-2_f64 * t3447 * t18416 * t18427 - 0.11851851851851851852e-1_f64 * t15376 * t22063 + 0.11851851851851851852e-1_f64 * t15376 * t22066 - 0.51851851851851851851e-2_f64 * t3447 * t15395 * t78035 + 0.34567901234567901234e-2_f64 * t3447 * t52100 * t73496 - 0.39506172839506172838e-2_f64 * t73272;
    (t78364, t78367, t78370, t78373, t78379, t78423)
}
