//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2731/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731(t1372: f64, t6387: f64, t6414: f64, t12259: f64, t1336: f64, t1352: f64, t1380: f64, t16033: f64, t16060: f64, t16065: f64, t16068: f64, t16416: f64, t1825: f64, t19654: f64, t19674: f64, t19761: f64, t19810: f64, t3777: f64, t5230: f64, t5234: f64, t5250: f64, t5333: f64, t5334: f64, t5336: f64, t5339: f64, t5341: f64, t5344: f64, t55039: f64, t57354: f64, t6420: f64) -> (f64, f64, f64) {
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57631 = -t12259 * t1336 * t6420 - 2.0_f64 * t1336 * t1380 * t57354 - 2.0_f64 * t1336 * t1825 * t55039 - 2.0_f64 * t1352 * t5344 * t57607 + 8.0_f64 * t5230 * t5333 * t5336 + 4.0_f64 * t5250 * t5334 * t57618 - 2.0_f64 * t16033 * t19761 - 4.0_f64 * t16060 * t5339 - 4.0_f64 * t16060 * t5341 + 4.0_f64 * t16065 * t19654 - 4.0_f64 * t16068 * t19810 - 4.0_f64 * t16416 * t5234 - 2.0_f64 * t19674 * t3777;
    (t57607, t57618, t57631)
}
