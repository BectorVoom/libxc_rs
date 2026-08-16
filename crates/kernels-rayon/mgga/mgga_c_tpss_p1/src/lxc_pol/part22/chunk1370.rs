//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1370/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1370(t1844: f64, t30367: f64, t5798: f64, t645: f64, t507: f64, t6435: f64, t12679: f64, t13244: f64, t1339: f64, t13965: f64, t1760: f64, t1800: f64, t18539: f64, t18547: f64, t18628: f64, t18690: f64, t18898: f64, t19001: f64, t19305: f64, t19579: f64, t19609: f64, t19620: f64, t20221: f64, t20357: f64, t24128: f64, t25469: f64, t3166: f64, t3493: f64, t3502: f64, t44045: f64, t44070: f64, t5801: f64, t5809: f64, t61801: f64, t6243: f64, t626: f64, t6323: f64, t65085: f64, t65094: f64, t65097: f64, t65899: f64, t65924: f64) -> (f64, f64) {
    let t67246 = t1844 * t30367;
    let t67250 = t5798 * t645;
    let t67270 = t507 * t6435;
    let t67274 = -2.0_f64 * t626 * t3166 * t6323 - 2.0_f64 * t5801 * t13244 - 6.0_f64 * t18547 * t24128 * t19609 - 6.0_f64 * t61801 * t20221 + 6.0_f64 * t18547 * t20357 * t44045 - 6.0_f64 * t18547 * t18690 * t44070 - 6.0_f64 * t18547 * t25469 * t12679 - 6.0_f64 * t18547 * t24128 * t13965 - 6.0_f64 * t19579 * t67246 * t65899 - 4.0_f64 * t67250 * t1339 - 4.0_f64 * t18898 * t3502 - 2.0_f64 * t3493 * t18628 - 2.0_f64 * t65094 * t1800 - 4.0_f64 * t65097 * t1800 - 4.0_f64 * t19305 * t5809 + t6243 * t19001 - 6.0_f64 * t19620 * t18690 * t65924 + 12.0_f64 * t18547 * t20357 * t65085 + 6.0_f64 * t1760 * t67270 * t18539;
    (t67250, t67274)
}
