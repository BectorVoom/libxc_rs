//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3189/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189(t11719: f64, t11728: f64, t11738: f64, t15545: f64, t15620: f64, t15625: f64, t15656: f64, t18303: f64, t19056: f64, t3248: f64, t3506: f64, t3509: f64, t3516: f64, t3577: f64, t3578: f64, t44896: f64, t44968: f64, t44972: f64, t44976: f64, t4582: f64, t5024: f64, t52991: f64, t52993: f64, t52999: f64, t53001: f64, t6219: f64) -> f64 {
    let t66219 = -t52991 / 972.0_f64 - t52993 / 3456.0_f64 + t52999 / 648.0_f64 + t44896 * t18303 / 256.0_f64 + t53001 / 576.0_f64 + t44968 / 10368.0_f64 + t44972 / 20736.0_f64 + t44976 / 10368.0_f64 + t3506 * t4582 * t19056 * t15620 / 1536.0_f64 + t11719 * t4582 * t19056 * t15625 / 512.0_f64 - t11728 * t4582 * t19056 * t3509 / 512.0_f64 - 5.0_f64 / 1296.0_f64 * t5024 * t15545 - 5.0_f64 / 216.0_f64 * t5024 * t15656 - t3577 * t3578 * t6219 * t3248 / 2304.0_f64 + t11738 * t4582 * t19056 * t3516 / 3072.0_f64;
    t66219
}
