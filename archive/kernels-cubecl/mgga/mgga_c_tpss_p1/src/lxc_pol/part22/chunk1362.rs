//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1362/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1362<F: Float>(t10085: F, t1838: F, t3259: F, t6419: F, t1232: F, t1265: F, t18483: F, t18490: F, t18511: F, t18964: F, t18967: F, t18976: F, t18981: F, t18994: F, t19509: F, t19540: F, t19554: F, t20154: F, t20174: F, t20178: F, t20179: F, t20190: F, t3260: F, t43908: F, t4459: F, t520: F, t5739: F, t5740: F, t5745: F, t5918: F, t60649: F, t60778: F, t62508: F, t6430: F, t65703: F, t65729: F, t65738: F, t65878: F, t65882: F) -> F {
    let t67006 = t10085 * t1838;
    let t67032 = t6419 * t3259;
    let t67057 = F::cast_from(4.0_f64) * t5739 * t5740 * t20154 * t1265 - F::cast_from(2.0_f64) * t19540 * t20190 * t65729 + F::cast_from(6.0_f64) * t19540 * t67006 * t65878 - F::cast_from(6.0_f64) * t19540 * t20190 * t65882 + F::cast_from(2.0_f64) * t19540 * t18967 * t43908 + t19540 * t18967 * t65738 - F::cast_from(4.0_f64) * t60649 * t20174 + t19540 * t18967 * t65703 + F::cast_from(2.0_f64) * t19540 * t62508 * t19554 + F::cast_from(2.0_f64) * t5739 * t5745 * t20154 * t1232 * t520 + F::cast_from(2.0_f64) * t19509 * t18976 - F::cast_from(2.0_f64) * t5739 * t18511 * t67032 * t3260 - F::cast_from(2.0_f64) * t19509 * t18981 + t19509 * t18994 + t60778 * t6430 - F::cast_from(6.0_f64) * t19509 * t18964 + F::cast_from(4.0_f64) * t18483 * t20179 + F::cast_from(2.0_f64) * t5739 * t5745 * t5918 * t4459 * t520 - F::cast_from(12.0_f64) * t5739 * t18490 * t20178 * t1265 + t5739 * t5745 * t67032 * t520;
    t67057
}
