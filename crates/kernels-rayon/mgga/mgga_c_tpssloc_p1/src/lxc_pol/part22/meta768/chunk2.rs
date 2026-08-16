//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2604/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604(t15503: f64, t18356: f64, t18975: f64, t5024: f64, t1174: f64, t21749: f64, t3431: f64, t11738: f64, t15569: f64, t15740: f64, t1735: f64, t18225: f64, t18300: f64, t18321: f64, t18387: f64, t18969: f64, t19068: f64, t3577: f64, t3578: f64, t4582: f64, t4950: f64, t4954: f64, t4969: f64, t4980: f64, t5012: f64, t65541: f64, t65815: f64, t65935: f64) -> f64 {
    let t72632 = t15503 * t18356;
    let t72634 = t5024 * t18975;
    let t72648 = t1174 * t3431 * t21749;
    let t72654 = t15569 * t18387 / 144.0_f64 - t65815 * t4954 / 1536.0_f64 - t15740 * t18969 / 1536.0_f64 - t65815 * t4950 / 1536.0_f64 - t72632 / 144.0_f64 - 5.0_f64 / 1296.0_f64 * t72634 - 5.0_f64 / 20736.0_f64 * t65935 + 19.0_f64 / 288.0_f64 * t65541 * t4980 - t3577 * t3578 * t1735 * t18225 / 384.0_f64 + t11738 * t4582 * t18300 * t5012 / 1024.0_f64 - t72648 / 144.0_f64 - 11.0_f64 / 54.0_f64 * t18321 * t4969 - 5.0_f64 / 864.0_f64 * t5024 * t19068;
    t72654
}
