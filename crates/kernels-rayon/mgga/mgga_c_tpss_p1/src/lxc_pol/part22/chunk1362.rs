//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1362/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1362(t10085: f64, t1838: f64, t3259: f64, t6419: f64, t1232: f64, t1265: f64, t18483: f64, t18490: f64, t18511: f64, t18964: f64, t18967: f64, t18976: f64, t18981: f64, t18994: f64, t19509: f64, t19540: f64, t19554: f64, t20154: f64, t20174: f64, t20178: f64, t20179: f64, t20190: f64, t3260: f64, t43908: f64, t4459: f64, t520: f64, t5739: f64, t5740: f64, t5745: f64, t5918: f64, t60649: f64, t60778: f64, t62508: f64, t6430: f64, t65703: f64, t65729: f64, t65738: f64, t65878: f64, t65882: f64) -> f64 {
    let t67006 = t10085 * t1838;
    let t67032 = t6419 * t3259;
    let t67057 = 4.0_f64 * t5739 * t5740 * t20154 * t1265 - 2.0_f64 * t19540 * t20190 * t65729 + 6.0_f64 * t19540 * t67006 * t65878 - 6.0_f64 * t19540 * t20190 * t65882 + 2.0_f64 * t19540 * t18967 * t43908 + t19540 * t18967 * t65738 - 4.0_f64 * t60649 * t20174 + t19540 * t18967 * t65703 + 2.0_f64 * t19540 * t62508 * t19554 + 2.0_f64 * t5739 * t5745 * t20154 * t1232 * t520 + 2.0_f64 * t19509 * t18976 - 2.0_f64 * t5739 * t18511 * t67032 * t3260 - 2.0_f64 * t19509 * t18981 + t19509 * t18994 + t60778 * t6430 - 6.0_f64 * t19509 * t18964 + 4.0_f64 * t18483 * t20179 + 2.0_f64 * t5739 * t5745 * t5918 * t4459 * t520 - 12.0_f64 * t5739 * t18490 * t20178 * t1265 + t5739 * t5745 * t67032 * t520;
    t67057
}
